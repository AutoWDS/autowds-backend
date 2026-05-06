use crate::model::{
    account_user,
    prelude::AccountUser,
    sea_orm_active_enums::{OrderLevel, ProductEdition},
};
use anyhow::Result;
use chrono::{Duration, Local};
use sea_orm::DbConn;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use summer::{plugin::service::Service, tracing};

#[derive(Clone, Service)]
pub struct UserService {
    #[inject(component)]
    pub db: DbConn,
}

impl UserService {
    /// 若会员已过期则自动降级为免费版（L0）。
    ///
    /// - `vip_expired_at` 为空时：不做过期判断（兼容历史“没有过期字段”的已付费用户）。
    pub async fn refresh_user_membership_by_id(&self, user_id: i64) -> Result<account_user::Model> {
        let user = AccountUser::find_by_id(user_id).one(&self.db).await?;
        let Some(user) = user else {
            anyhow::bail!("用户不存在: user_id={user_id}");
        };
        self.refresh_user_membership(user).await
    }

    pub async fn refresh_user_membership(
        &self,
        user: account_user::Model,
    ) -> Result<account_user::Model> {
        if user.edition == ProductEdition::L0 {
            return Ok(user);
        }
        let Some(expired_at) = user.vip_expired_at else {
            // 历史数据：不做过期降级，避免无意中清退存量会员
            return Ok(user);
        };
        let now = Local::now().naive_local();
        if expired_at > now {
            return Ok(user);
        }

        let updated = account_user::ActiveModel {
            id: Set(user.id),
            edition: Set(ProductEdition::L0),
            vip_expired_at: Set(None),
            ..Default::default()
        }
        .update(&self.db)
        .await?;

        Ok(updated)
    }

    /// 确认用户支付，更新用户会员状态
    pub async fn confirm_user(
        &self,
        user_id: i64,
        level: OrderLevel,
        edition: ProductEdition,
    ) -> Result<String> {
        tracing::info!(
            "确认用户 {} 的 {:?} 会员支付，edition={:?}",
            user_id,
            level,
            edition
        );

        let user = AccountUser::find_by_id(user_id).one(&self.db).await?;
        let Some(user) = user else {
            anyhow::bail!("用户不存在: user_id={user_id}");
        };

        let now = Local::now().naive_local();
        let base = user.vip_expired_at.filter(|t| *t > now).unwrap_or(now);
        let new_expired_at = match level {
            OrderLevel::Monthly => base + Duration::days(30),
            OrderLevel::Annual => base + Duration::days(365),
        };

        account_user::ActiveModel {
            id: Set(user_id),
            edition: Set(edition),
            vip_expired_at: Set(Some(new_expired_at)),
            ..Default::default()
        }
        .update(&self.db)
        .await?;

        Ok(format!(
            "用户 {user_id} 的 {:?} 会员已激活，到期时间：{}",
            level, new_expired_at
        ))
    }
}
