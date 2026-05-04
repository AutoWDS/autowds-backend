use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::Serialize;
use summer_sqlx::sqlx::{postgres::PgRow, Row};

/// 单条采集记录（对应 `task_instance_record_{userId}` 一行）。
#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TaskInstanceCaptureItem {
    pub id: i64,
    pub task_id: i64,
    pub task_instance_id: i64,
    pub dedupe_rule_version: u32,
    pub dedupe_key: Option<String>,
    pub payload: serde_json::Value,
    pub created_at: String,
}

impl TaskInstanceCaptureItem {
    pub fn try_from_row(row: &PgRow) -> Result<Self, summer_sqlx::sqlx::Error> {
        let created: DateTime<Utc> = row.try_get("created_at")?;
        Ok(Self {
            id: row.try_get("id")?,
            task_id: row.try_get("task_id")?,
            task_instance_id: row.try_get("task_instance_id")?,
            dedupe_rule_version: row.try_get::<i32, _>("dedupe_rule_version")? as u32,
            dedupe_key: row.try_get("dedupe_key")?,
            payload: row.try_get::<serde_json::Value, _>("payload")?,
            created_at: created.to_rfc3339(),
        })
    }
}
