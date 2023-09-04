use super::enums::ProductEdition;
use super::enums::TemplateTopic;
use super::page::Page;
use super::page::PageRequest;
use chrono::NaiveDateTime;
use ormlitex::{model::*, Result};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use sqlx::types::Json;
use sqlx::PgPool;

#[derive(Debug, Model)]
#[ormlitex(table = "task_template")]
pub struct TaskTemplate {
    #[ormlitex(primary_key)]
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
    ) -> Result<Page<TaskTemplate>> {
        let total = 10;
        // TODO
        //let total =

        let content = TaskTemplate::select()
            .where_bind_option("name like '%?'", query.name.as_ref())
            .where_bind_option("topic=?", query.topic.as_ref())
            .where_bind_option("edition=?", query.edition.as_ref())
            .offset(page.offset())
            .limit(page.limit())
            .fetch_all(db)
            .await?;

        Ok(Page::new(content, total, page))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateQuery {
    name: Option<String>,
    topic: Option<TemplateTopic>,
    edition: Option<ProductEdition>,
}
