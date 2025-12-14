use crate::{
    config::mail::Email,
    model::{account_user, prelude::AccountUser, sea_orm_active_enums::{ProductEdition, CreditOperation}},
    router::ClientIp,
    utils::{
        credit::CreditService,
        jwt::{self, Claims},
        mail,
        validate_code::{gen_validate_code, get_validate_code},
    },
    views::user::{
        CreditLogResp, RegisterReq, ResetPasswdReq, SendEmailReq, SetNameReq, UserResp, ValidateCodeEmailTemplate,
    },
};
use anyhow::Context;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait};
use spring_mail::Mailer;
use spring_redis::Redis;
use spring_sea_orm::DbConn;
use spring_web::{
    axum::Json,
    error::{KnownWebError, Result},
    extractor::Component,
};
use spring_web::{extractor::Config, get_api, patch_api, post_api};

/// # 注册
/// @tag user
#[post_api("/user")]
async fn register(
    Component(mut redis): Component<Redis>,
    Component(db): Component<DbConn>,
    ClientIp(client_ip): ClientIp,
    Json(body): Json<RegisterReq>,
) -> Result<Json<UserResp>> {
    let code = get_validate_code(&mut redis, &body.email).await?;

    match code {
        None => return Err(KnownWebError::bad_request("验证码已过期"))?,
        Some(code) => {
            if code != body.validate_code {
                return Err(KnownWebError::bad_request("验证码错误"))?;
            }
        }
    }

    let user = AccountUser::find()
        .filter(account_user::Column::Email.eq(&body.email))
        .one(&db)
        .await
        .context("select user from db failed")?;
    if user.is_some() {
        return Err(KnownWebError::bad_request("邮箱已被注册"))?;
    }

    // 处理邀请码
    let mut invited_by = None;
    if let Some(invite_code) = &body.invite_code {
        let inviter = AccountUser::find()
            .filter(account_user::Column::InviteCode.eq(invite_code))
            .one(&db)
            .await
            .context("查询邀请人失败")?;
        
        if let Some(inviter) = inviter {
            invited_by = Some(inviter.id);
        } else {
            return Err(KnownWebError::bad_request("邀请码无效"))?;
        }
    }


    // 使用事务确保原子性
    let txn = db.begin().await.context("开始事务失败")?;
    
    // 先插入用户获取ID
    let user = account_user::ActiveModel {
        id: NotSet,
        locked: Set(false),
        edition: Set(ProductEdition::L0),
        name: Set(body.name),
        email: Set(body.email),
        passwd: Set(body.passwd),
        last_login: Set(Some(client_ip.0.into())),
        credits: Set(100), // 默认100积分
        invite_code: Set("TEMP".to_string()), // 临时邀请码，获取ID后立即更新
        invited_by: Set(invited_by),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .context("user insert failed")?;

    // 立即生成邀请码并更新
    let invite_code = CreditService::generate_invite_code(user.id);
    let mut user_active: account_user::ActiveModel = user.clone().into();
    user_active.invite_code = Set(invite_code);
    let user = user_active.update(&txn).await.context("更新邀请码失败")?;

    // 记录注册积分日志
    CreditService::add_credits(
        &txn,
        user.id,
        100,
        CreditOperation::Register,
        Some("注册奖励".to_string()),
        None,
    )
    .await
    .context("记录注册积分失败")?;

    // 如果有邀请人，给邀请人增加积分
    if let Some(inviter_id) = invited_by {
        CreditService::handle_invite_register(&txn, inviter_id, user.id)
            .await
            .context("处理邀请奖励失败")?;
    }
    
    // 提交事务
    txn.commit().await.context("提交事务失败")?;

    Ok(Json(user.into()))
}

/// # 获取当前用户信息
/// @tag user
#[get_api("/user")]
async fn current_user(claims: Claims, Component(db): Component<DbConn>) -> Result<Json<UserResp>> {
    let user = AccountUser::find_by_id(claims.uid)
        .one(&db)
        .await
        .with_context(|| format!("find user by id#{}", claims.uid))?
        .ok_or_else(|| KnownWebError::not_found("用户不存在"))?;

    if user.email != claims.email {
        Err(KnownWebError::forbidden("Token数据有误"))?;
    }

    Ok(Json(UserResp::from(user)))
}

/// # 注册验证码
/// @tag user
#[post_api("/user/register-validate-code")]
async fn register_validate_code(
    Component(mut redis): Component<Redis>,
    Component(mailer): Component<Mailer>,
    Config(email): Config<Email>,
    Json(body): Json<SendEmailReq>,
) -> Result<Json<bool>> {
    let code = gen_validate_code(&mut redis, &body.email).await?;

    let template = ValidateCodeEmailTemplate {
        tip: "欢迎您注册我们的服务，您的注册验证码(5分钟内有效)是：",
        code: code.as_str(),
    };
    let from = email.from;
    let to = body.email;
    let success = mail::send_mail(&mailer, &from, &to, "注册验证码", &template).await?;

    Ok(Json(success))
}

/// # 重置验证码
/// @tag user
#[post_api("/user/reset-validate-code")]
async fn reset_validate_code(
    Component(mut redis): Component<Redis>,
    Component(mailer): Component<Mailer>,
    Config(email): Config<Email>,
    Json(body): Json<SendEmailReq>,
) -> Result<Json<bool>> {
    let code = gen_validate_code(&mut redis, &body.email).await?;

    let template = ValidateCodeEmailTemplate {
        tip: "请确认您是否需要重置密码，重置密码请在系统中输入以下验证码(5分钟内有效)：",
        code: code.as_str(),
    };
    let from = email.from;
    let to = body.email;
    let success = mail::send_mail(&mailer, &from, &to, "重置密码的验证码", &template).await?;

    Ok(Json(success))
}

/// # 重置密码
/// @tag user
#[post_api("/user/passwd")]
async fn reset_password(
    Component(mut redis): Component<Redis>,
    Component(db): Component<DbConn>,
    ClientIp(client_ip): ClientIp,
    Json(req): Json<ResetPasswdReq>,
) -> Result<String> {
    let code = get_validate_code(&mut redis, &req.email)
        .await?
        .ok_or_else(|| KnownWebError::bad_request("验证码已过期"))?;

    if code != req.validate_code {
        Err(KnownWebError::bad_request("验证码错误"))?;
    }

    let u = AccountUser::find()
        .filter(account_user::Column::Email.eq(&req.email))
        .one(&db)
        .await
        .with_context(|| format!("query user by email failed: {}", req.email))?
        .ok_or_else(|| KnownWebError::not_found("用户不存在"))?;

    let u = account_user::ActiveModel {
        id: Set(u.id),
        passwd: Set(req.passwd),
        last_login: Set(Some(client_ip.0.into())),
        ..Default::default()
    }
    .update(&db)
    .await
    .with_context(|| format!("user#{} change password failed", u.id))?;

    let claims = Claims::new(u);

    Ok(jwt::encode(claims)?)
}

/// # 修改用户名
/// @tag user
#[patch_api("/user/name")]
async fn set_name(
    claims: Claims,
    Component(db): Component<DbConn>,
    Json(req): Json<SetNameReq>,
) -> Result<Json<bool>> {
    let u = AccountUser::find_by_id(claims.uid)
        .one(&db)
        .await
        .with_context(|| format!("query user by id#{} failed", claims.uid))?
        .ok_or_else(|| KnownWebError::not_found("用户不存在"))?;

    if claims.email != u.email {
        Err(KnownWebError::forbidden("Token数据有误"))?;
    }

    let u = account_user::ActiveModel {
        id: Set(u.id),
        name: Set(req.name),
        ..Default::default()
    }
    .update(&db)
    .await
    .with_context(|| format!("change name for user#{} failed", u.id))?;

    tracing::debug!("user#{} change name success", u.id);

    Ok(Json(true))
}
/// # 数据导出（扣减积分）
/// @tag user
#[post_api("/user/export")]
async fn export_data(
    claims: Claims,
    Component(db): Component<DbConn>,
) -> Result<Json<bool>> {
    // 扣减1个积分
    CreditService::deduct_credits(
        &db,
        claims.uid,
        1,
        CreditOperation::Export,
        Some("数据导出".to_string()),
    )
    .await
    .map_err(|e| {
        if e.to_string().contains("积分不足") {
            KnownWebError::bad_request("积分不足，无法导出数据")
        } else {
            KnownWebError::internal_server_error("扣减积分失败")
        }
    })?;

    Ok(Json(true))
}

/// # 获取积分记录
/// @tag user  
#[get_api("/user/credits/logs")]
async fn get_credit_logs(
    claims: Claims,
    Component(db): Component<DbConn>,
) -> Result<Json<Vec<CreditLogResp>>> {
    use crate::model::prelude::CreditLog;
    use crate::model::credit_log;

    let logs = CreditLog::find()
        .filter(credit_log::Column::UserId.eq(claims.uid))
        .order_by_desc(credit_log::Column::Created)
        .limit(50)
        .all(&db)
        .await
        .context("查询积分记录失败")?;

    let resp: Vec<CreditLogResp> = logs.into_iter().map(|log| log.into()).collect();
    Ok(Json(resp))
}