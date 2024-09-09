//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::ProductEdition;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "account_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub created: DateTime,
    pub modified: DateTime,
    pub edition: ProductEdition,
    pub name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub passwd: String,
    pub locked: bool,
    #[sea_orm(column_type = "custom(\"inet\")", nullable)]
    pub last_login: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
