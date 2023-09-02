use chrono::NaiveDateTime;
use ormlite::{model::*, types::Json};
use serde_json::Value;

use super::enums::StoreType;

#[derive(Clone, Debug, Model)]
#[ormlite(table = "data_store_meta")]
pub struct DataStoreMeta {
    #[ormlite(primary_key)]
    pub id: i64,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub user_id: i64,
    pub count: i64,
    pub name: String,
    #[ormlite(column = "type")]
    pub stype: StoreType,
    pub config: Json<Value>,
}
