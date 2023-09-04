use chrono::NaiveDateTime;
use ormlitex::model::*;
use serde_json::Value;
use sqlx::types::Json;

#[derive(Debug, Model)]
#[ormlitex(table = "scraper_task")]
pub struct ScraperTask {
    #[ormlitex(primary_key)]
    pub id: i64,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub user_id: i64,
    pub deleted: bool,
    pub name: String,
    pub rule: Json<Value>,
}