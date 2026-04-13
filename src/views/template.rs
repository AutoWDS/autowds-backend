use crate::model::{
    sea_orm_active_enums::{ProductEdition, TemplateTopic},
    task_template,
};
use schemars::JsonSchema;
use sea_orm::{prelude::DateTime, ColumnTrait, Condition};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::serde_as;
use serde_with::NoneAsEmptyString;
use validator::Validate;

/// 模板列表允许的排序字段（与 ORM `Column` 解耦，便于 OpenAPI 与校验）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum TemplateListSort {
    /// 按更新时间
    #[default]
    Modified,
    /// 按收藏数（热度）
    FavCount,
}

impl TemplateListSort {
    pub fn column(self) -> task_template::Column {
        match self {
            Self::Modified => task_template::Column::Modified,
            Self::FavCount => task_template::Column::FavCount,
        }
    }
}

/// # 模板查询
#[serde_as]
#[derive(Deserialize, Validate, JsonSchema)]
pub struct TemplateQuery {
    /// # 模板名称
    #[validate(length(max = 30, message = "查询名称过长"))]
    pub name: Option<String>,
    /// # 主题
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    #[schemars(with = "Option<TemplateTopic>")]
    pub topic: Option<TemplateTopic>,
    /// # 产品授权级别
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(default)]
    #[schemars(with = "Option<ProductEdition>")]
    pub edition: Option<ProductEdition>,
    /// # 排序（查询参数 `sort`，与扩展端一致）
    #[serde(default)]
    pub sort: TemplateListSort,
}

impl From<TemplateQuery> for Condition {
    fn from(query: TemplateQuery) -> Self {
        let mut c = Condition::all();
        if let Some(name) = query.name {
            c = c.add(task_template::Column::Name.starts_with(name));
        }
        if let Some(topic) = query.topic {
            c = c.add(task_template::Column::Topic.eq(topic));
        }
        if let Some(edition) = query.edition {
            c = c.add(task_template::Column::Edition.eq(edition));
        }
        c
    }
}

/// # 预制模板列表
#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
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
    /// 采集规则（列表接口需返回，供扩展端「应用模板」创建任务）
    rule: Value,
    /// 起始 URL 中 `{参数名}` 占位符列表，与 rule 内 start 节点 url 对应
    params: Option<Value>,
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
            rule: value.rule,
            params: value.params,
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
