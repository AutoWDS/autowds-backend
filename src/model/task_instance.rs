use super::enums::TaskStatus;
use chrono::NaiveDateTime;
use ormlite::model::*;
use serde_json::Value;
use sqlx::types::Json;

#[derive(Debug, Model)]
#[ormlite(table = "task_instance")]
pub struct TaskInstance {
    #[ormlite(primary_key)]
    pub id: i64,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub user_id: i64,
    pub task_id: i64,
    pub status: TaskStatus,
    pub rule: Json<Value>,
    pub version: i64,
}
