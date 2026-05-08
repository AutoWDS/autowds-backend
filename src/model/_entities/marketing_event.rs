use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "marketing_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub campaign_id: Option<i64>,
    pub delivery_id: Option<i64>,
    pub lead_id: Option<i64>,
    pub event_type: String,
    pub url: Option<String>,
    pub user_agent: Option<String>,
    pub ip_hash: Option<String>,
    pub created_at: DateTime,
    #[sea_orm(column_type = "JsonBinary")]
    pub meta: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
