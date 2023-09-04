use chrono::NaiveDateTime;
use ormlitex::{model::*, types::Json};
use serde_json::Value;

use super::enums::StoreType;

#[derive(Clone, Debug, Model)]
#[ormlitex(table = "data_store_meta")]
pub struct DataStoreMeta {
    #[ormlitex(primary_key)]
    pub id: i64,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub user_id: i64,
    pub count: i64,
    pub name: String,
    #[ormlitex(column = "type")]
    pub stype: StoreType,
    pub config: Json<Value>,
}
