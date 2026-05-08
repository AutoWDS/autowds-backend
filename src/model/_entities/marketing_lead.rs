use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "marketing_lead")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub email: String,
    pub name: Option<String>,
    pub source: Option<String>,
    #[sea_orm(column_type = "JsonBinary")]
    pub tags: Option<Json>,
    #[sea_orm(column_type = "JsonBinary")]
    pub extra: Option<Json>,
    pub status: String,
    pub unsubscribed: bool,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    pub last_seen_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
