//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::ProductEdition;
use super::sea_orm_active_enums::TemplateTopic;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "task_template")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub created: DateTime,
    pub modified: DateTime,
    pub topic: TemplateTopic,
    pub edition: ProductEdition,
    pub lang: String,
    pub fav_count: i32,
    pub name: String,
    pub detail: String,
    pub img: String,
    #[sea_orm(column_type = "JsonBinary")]
    pub rule: Json,
    #[sea_orm(column_type = "JsonBinary")]
    pub data: Json,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub params: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
