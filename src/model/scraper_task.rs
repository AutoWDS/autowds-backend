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
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq, FromJsonQueryResult, JsonSchema)]
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

/// 用于比较「去重路径是否变化」：去空、排序、去重。
pub fn normalized_dedupe_json_paths(paths: &[String]) -> Vec<String> {
    let mut v: Vec<String> = paths
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    v.sort();
    v.dedup();
    v
}

/// 写入前根据路径集合相对 `prev_task_data` 是否变化，校正 `dedupe_rule_version`（**忽略**请求里客户端自带的版本号）。
pub fn apply_data_quality_dedupe_version(
    prev_task_data: Option<&ScraperTaskData>,
    dq: &mut DataQualityConfig,
) {
    let old_paths = prev_task_data
        .and_then(|d| d.data_quality.as_ref())
        .map(|q| normalized_dedupe_json_paths(&q.dedupe_json_paths))
        .unwrap_or_default();
    let new_paths = normalized_dedupe_json_paths(&dq.dedupe_json_paths);
    let old_ver = prev_task_data
        .and_then(|d| d.data_quality.as_ref())
        .map(|q| q.dedupe_rule_version)
        .unwrap_or(0);
    if new_paths != old_paths {
        dq.dedupe_rule_version = old_ver.saturating_add(1);
    } else {
        dq.dedupe_rule_version = old_ver;
    }
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

#[cfg(test)]
mod dedupe_version_tests {
    use super::*;

    fn dq(paths: &[&str], ver: u32) -> DataQualityConfig {
        DataQualityConfig {
            dedupe_rule_version: ver,
            dedupe_json_paths: paths.iter().map(|s| (*s).to_string()).collect(),
        }
    }

    fn task_data(paths: &[&str], ver: u32) -> ScraperTaskData {
        ScraperTaskData {
            schedule: None,
            data_quality: Some(dq(paths, ver)),
        }
    }

    #[test]
    fn paths_unchanged_keeps_version() {
        let prev = task_data(&["a", "b"], 3);
        let mut incoming = dq(&["b", "a"], 999);
        apply_data_quality_dedupe_version(Some(&prev), &mut incoming);
        assert_eq!(incoming.dedupe_rule_version, 3);
    }

    #[test]
    fn paths_changed_bumps_version() {
        let prev = task_data(&["a"], 2);
        let mut incoming = dq(&["a", "c"], 0);
        apply_data_quality_dedupe_version(Some(&prev), &mut incoming);
        assert_eq!(incoming.dedupe_rule_version, 3);
    }

    #[test]
    fn create_without_prev_nonempty_paths_starts_at_one() {
        let mut incoming = dq(&["x"], 0);
        apply_data_quality_dedupe_version(None, &mut incoming);
        assert_eq!(incoming.dedupe_rule_version, 1);
    }

    #[test]
    fn create_empty_paths_stays_zero() {
        let mut incoming = dq(&[], 5);
        apply_data_quality_dedupe_version(None, &mut incoming);
        assert_eq!(incoming.dedupe_rule_version, 0);
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
