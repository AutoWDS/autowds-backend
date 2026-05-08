use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "marketing_delivery")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub campaign_id: i64,
    pub lead_id: i64,
    pub email: String,
    #[sea_orm(unique)]
    pub token: String,
    pub status: String,
    pub sent_at: Option<DateTime>,
    pub provider_task_id: Option<String>,
    pub provider_message_id: Option<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime,
    pub modified_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
