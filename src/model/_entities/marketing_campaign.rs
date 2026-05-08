use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "marketing_campaign")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    pub subject: String,
    pub landing_url: String,
    pub status: String,
    pub created_by: i64,
    pub created_at: DateTime,
    pub modified_at: DateTime,
    pub scheduled_at: Option<DateTime>,
    pub provider_receiver_id: Option<String>,
    pub provider_template_id: Option<String>,
    pub provider_task_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
