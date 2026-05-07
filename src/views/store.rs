use chrono::{DateTime, NaiveDateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use summer_sqlx::sqlx::{postgres::PgRow, Row};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DatasetQuery {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DatasetMeta {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub created: i64,
    pub count: i64,
    pub bytes: i64,
    #[serde(rename = "type")]
    pub ty: DatasetType,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub enum DatasetType {
    DOC,
    RDB,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DatasetDataQuery {
    pub offset: Option<i64>,
    #[serde(default)]
    pub desc: bool,
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DatasetDataPage {
    pub content: Vec<DatasetDataItem>,
    pub total: i64,
    pub size: i64,
    pub offset: i64,
    pub desc: bool,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DatasetDataItem {
    pub id: String,
    pub created: String,
    pub modified: String,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DatasetField {
    pub name: String,
    pub default_value: String,
}

impl DatasetDataItem {
    pub fn try_from_row(row: &PgRow) -> Result<Self, summer_sqlx::sqlx::Error> {
        let id: i64 = row.try_get("id")?;
        let created: DateTime<Utc> = row.try_get("created_at")?;
        let data: Value = row.try_get("payload")?;
        let created = created.to_rfc3339();
        Ok(Self {
            id: id.to_string(),
            created: created.clone(),
            modified: created,
            data,
        })
    }
}

pub fn dataset_created_millis(created: NaiveDateTime) -> i64 {
    created.and_utc().timestamp_millis()
}
