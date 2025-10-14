pub use super::_entities::scraper_task::*;

use anyhow::Context;
use schemars::JsonSchema;
use sea_orm::{
    sqlx::types::chrono::Local, ActiveModelBehavior, ConnectionTrait, DbConn, DbErr, EntityTrait,
    FromJsonQueryResult, Set,
};
use serde::{Deserialize, Serialize};
use spring::async_trait;
use spring_web::error::{KnownWebError, WebError};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum ScheduleType {
    Fast,
    Browser,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, FromJsonQueryResult, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub struct ScheduleData {
    pub cron: String,
    pub proxy_id: i32,
    pub ty: ScheduleType,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.created = Set(Local::now().naive_local());
            self.deleted = Set(false);
        }
        self.modified = Set(Local::now().naive_local());
        Ok(self)
    }
}

impl Entity {
    pub async fn find_check_task(db: &DbConn, id: i64, uid: i64) -> Result<Model, WebError> {
        let task = Entity::find_by_id(id)
            .one(db)
            .await
            .context("find scraper task failed")?
            .ok_or_else(|| KnownWebError::not_found("任务不存在"))?;

        if task.user_id != uid {
            Err(KnownWebError::forbidden("数据无权访问"))?;
        }

        Ok(task)
    }
}
