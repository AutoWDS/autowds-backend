use super::enums::ProductEdition;
use super::enums::TemplateTopic;
use chrono::NaiveDateTime;
use ormlite::model::*;
use serde_json::Value;
use sqlx::types::Json;

#[derive(Debug, Model)]
#[ormlite(table = "task_template")]
pub struct TaskTemplate {
    #[ormlite(primary_key)]
    pub id: i64,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub topic: TemplateTopic,
    pub edition: ProductEdition,
    pub lang: String,
    pub fav_count: i32,
    pub name: String,
    pub detail: String,
    pub img: String,
    pub rule: Json<Value>,
    pub data: Json<Value>,
    pub params: Option<Json<Value>>,
}
