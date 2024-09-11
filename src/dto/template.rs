use sea_orm::{prelude::DateTime, sea_query::IntoCondition, ColumnTrait, Condition};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use validator::Validate;
use serde_with::NoneAsEmptyString;

use crate::model::{
    sea_orm_active_enums::{ProductEdition, TemplateTopic},
    task_template,
};

#[serde_as]
#[derive(Deserialize, Validate)]
pub struct TemplateQuery {
    #[validate(length(max = 30, message = "查询名称过长"))]
    pub name: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    pub topic: Option<TemplateTopic>,
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    pub edition: Option<ProductEdition>,
}

impl IntoCondition for TemplateQuery {
    fn into_condition(self) -> sea_orm::Condition {
        let mut c = Condition::all();
        if let Some(name) = self.name {
            c = c.add(task_template::Column::Name.starts_with(name));
        }
        if let Some(topic) = self.topic {
            c = c.add(task_template::Column::Topic.eq(topic));
        }
        if let Some(edition) = self.edition {
            c = c.add(task_template::Column::Edition.eq(edition));
        }
        c
    }
}

#[derive(Debug, Serialize)]
pub struct ListTemplateResp {
    id: i64,
    created: DateTime,
    modified: DateTime,
    topic: TemplateTopic,
    edition: ProductEdition,
    lang: String,
    fav_count: i32,
    name: String,
    detail: String,
    img: String,
    like: bool,
}

impl From<task_template::Model> for ListTemplateResp {
    fn from(value: task_template::Model) -> Self {
        Self {
            id: value.id,
            created: value.created,
            modified: value.modified,
            topic: value.topic,
            edition: value.edition,
            lang: value.lang,
            fav_count: value.fav_count,
            name: value.name,
            detail: value.detail,
            img: value.img,
            like: false,
        }
    }
}

impl ListTemplateResp {
    pub fn new(model: task_template::Model, like: bool) -> Self {
        let mut resp = ListTemplateResp::from(model);
        resp.like = like;
        resp
    }
}
