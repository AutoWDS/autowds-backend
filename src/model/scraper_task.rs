pub use super::_entities::scraper_task::*;

use anyhow::Context;
use chrono::Local;
use schemars::JsonSchema;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbConn, DbErr, EntityTrait, FromJsonQueryResult, Set,
};
use serde::{Deserialize, Serialize};
use summer::async_trait;
use summer_web::error::{KnownWebError, WebError};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum ScheduleType {
    Fast,
    Browser,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, FromJsonQueryResult, JsonSchema)]
pub struct ScheduleData {
    pub cron: String,
    #[serde(rename = "proxyId")]
    pub proxy_id: i32,
    #[serde(rename = "type")]
    pub ty: ScheduleType,
}

/// 落库在 `scraper_task.data` 一列：调度 + 数据质量等任务级配置（信封结构）。
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, FromJsonQueryResult, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScraperTaskData {
    /// 未配置调度时可为空，仅保留 `data_quality` 等配置。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ScheduleData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_quality: Option<DataQualityConfig>,
}

/// 数据质量 / 去重等与调度无关的配置；随 `dedupe_rule_version` 变更可换规则而不污染历史行语义。
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, FromJsonQueryResult, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DataQualityConfig {
    #[serde(default)]
    pub dedupe_rule_version: u32,
    /// 参与规范化去重材料的 JSON 路径（相对每条采集记录的根对象），如 `["url"]` 或 `["siteId","sku"]`。
    #[serde(default)]
    pub dedupe_json_paths: Vec<String>,
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
