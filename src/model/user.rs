use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "product_edition")]
pub enum ProductEdition {
    L0,
    L1,
    L2,
    L3,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct AccountUser {
    pub id: i64,
    pub locked: bool,
    pub edition: ProductEdition,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub passwd: String,
    pub last_login: String,
}
