use super::enums::TaskStatus;
use chrono::NaiveDateTime;
use ormlitex::model::*;
use serde_json::Value;
use sqlx::types::Json;

#[derive(Debug, Model)]
#[ormlitex(table = "task_instance")]
pub struct TaskInstance {
    #[ormlitex(primary_key)]
    pub id: i64,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub user_id: i64,
    pub task_id: i64,
    pub status: TaskStatus,
    pub rule: Json<Value>,
    pub version: i64,
}
