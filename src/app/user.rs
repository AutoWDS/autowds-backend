use std::net::IpAddr;

use actix_web::{error, post, web, HttpResponse, Responder, Scope};
use actix_web_validator::Json;
use deadpool_redis::redis;
use rbatis::rbdc::datetime::DateTime;
use rbatis::rbdc::timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::model::user::{self as user_model, AccountUser, ProductEdition};
use crate::utils::mail::{self, ValidateCodeMailTemplate};
use crate::utils::rand;
use crate::AppState;

pub fn user_scope() -> Scope {
    return web::scope("/user")
        .service(register)
        .service(reset_passwd)
        .service(register_validate_code)
        .service(reset_validate_code);
}

#[post("/")]
async fn register(
    state: web::Data<AppState>,
    ip_addr: Option<web::ReqData<IpAddr>>,
    body: Json<RegisterDTO>,
) -> Result<HttpResponse, error::Error> {
    let mut rb = &state.rbatis;
    let exists = user_model::exists_by_email(&rb, &body.email).await.unwrap();

    if exists {
        return Err(error::ErrorBadRequest("邮箱已被注册").into());
    }

    let success = AccountUser::insert(
        &mut rb,
        &AccountUser {
            id: None,
            locked: false,
            edition: ProductEdition::L0,
            email: String::from(&body.email),
            name: String::from(&body.name),
            passwd: String::from(&body.passwd),
            last_login: ip_addr.unwrap().into_inner(),
            created: DateTime::now(),
            modified: DateTime::now(),
        },
    )
    .await
    .unwrap();

    if success.rows_affected <= 0 {
        return Err(error::ErrorInternalServerError("注册失败").into());
    }

    return Ok(HttpResponse::Ok().json("注册成功"));
}

#[post("/passwd")]
async fn reset_passwd(
    state: web::Data<AppState>,
    ip_addr: Option<web::ReqData<IpAddr>>,
    body: Json<ResetPasswdDTO>,
) -> Result<HttpResponse, error::Error> {
    let mut rb = &state.rbatis;
    let user_optional = AccountUser::select_by_email(&mut rb, &body.email)
        .await
        .unwrap();

    let user = match user_optional {
        Some(u) => AccountUser {
            passwd: String::from(&body.passwd),
            // modified: NOW
            last_login: ip_addr.unwrap().into_inner(),
            ..u
        },
        None => return Err(error::ErrorNotFound("用户不存在").into()),
    };

    let success = AccountUser::update_by_id(&mut rb, &user, user.id.unwrap())
        .await
        .unwrap();

    if success.rows_affected <= 0 {
        return Err(error::ErrorBadRequest("修改失败").into());
    }

    return Ok(HttpResponse::Ok().json(user));
}

#[post("/register-validate-code")]
async fn register_validate_code(
    state: web::Data<AppState>,
    body: Json<SendEmailDTO>,
) -> impl Responder {
    let validate_code = generate_validate_code(&state, &body.email).await;
    let template = ValidateCodeMailTemplate {
        tip: "欢迎您注册我们的服务，您的注册验证码(5分钟内有效)是：",
        validate_code: validate_code.as_str(),
    };
    mail::send_mail(&body.email, "注册验证码", &template);
    ""
}

#[post("/reset-validate-code")]
async fn reset_validate_code(
    state: web::Data<AppState>,
    body: Json<SendEmailDTO>,
) -> impl Responder {
    let validate_code = generate_validate_code(&state, &body.email).await;
    let template = ValidateCodeMailTemplate {
        tip: "请确认您是否需要重置密码，重置密码请在系统中输入以下验证码(5分钟内有效)：",
        validate_code: validate_code.as_str(),
    };
    mail::send_mail(&body.email, "重置密码的验证码", &template);
    ""
}

async fn generate_validate_code(state: &AppState, email: &str) -> String {
    let rand_code = rand::rand_alphanumeric(6);
    let mut conn = state.redis.get().await.unwrap();
    redis::cmd("SETEX")
        .arg(format!("email-validate:{}", email))
        .arg(300) // 5min expire
        .arg(&rand_code)
        .query_async::<_, ()>(&mut conn)
        .await
        .unwrap();
    return rand_code;
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct RegisterDTO {
    #[validate(length(min = 1, max = 30, message = "用户名需在1-30个字符间"))]
    name: String,
    #[validate(email(message = "邮箱格式不正确"))]
    email: String,
    #[validate(length(min = 1, max = 30, message = "密码需在1-30个字符间"))]
    passwd: String,
    #[serde(rename = "validateCode")]
    validate_code: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct ResetPasswdDTO {
    #[validate(email(message = "邮箱格式不正确"))]
    email: String,
    #[validate(length(min = 1, max = 30, message = "密码需在1-30个字符间"))]
    passwd: String,
    #[serde(rename = "validateCode")]
    validate_code: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct SendEmailDTO {
    #[validate(email(message = "邮箱格式不正确"))]
    email: String,
}
