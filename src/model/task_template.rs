use super::enums::ProductEdition;
use super::enums::TemplateTopic;
use super::page::PageRequest;
use chrono::NaiveDateTime;
use ormlite::{model::*, Result};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use sqlx::types::Json;
use sqlx::PgPool;

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

impl TaskTemplate {
    pub async fn find_by_query(
        db: &PgPool,
        query: &TemplateQuery,
        page: &PageRequest,
    ) -> Result<Vec<TaskTemplate>> {
        let mut query_builder = TaskTemplate::select();
        if let Some(name) = &query.name {
            query_builder = query_builder.where_bind("name like '%?'", name);
        }
        if let Some(topic) = &query.topic {
            query_builder = query_builder.where_bind("topic=?", topic);
        }
        if let Some(edition) = &query.edition {
            query_builder = query_builder.where_bind("edition=?", edition);
        }
        query_builder
            .offset(page.offset())
            .limit(page.limit())
            .fetch_all(db)
            .await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateQuery {
    name: Option<String>,
    topic: Option<TemplateTopic>,
    edition: Option<ProductEdition>,
}
