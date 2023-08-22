use rbatis::{crud, impl_select, impl_update, sql, RBatis};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum ProductEdition {
    L0,
    L1,
    L2,
    L3,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountUser {
    pub id: Option<i64>,
    pub locked: bool,
    pub edition: ProductEdition,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub passwd: String,
    pub last_login: String,
}

crud!(AccountUser {});

impl_select!(AccountUser{select_by_email(email:&str)->Option => "`where email=#{email}`"});

impl_update!(AccountUser{update_by_id(id:i64)=>"`where id = 1`"});

#[sql("select count(*)>0 from account_user where email=#{email}")]
pub async fn exists_by_email(rb: &RBatis, email: &str) -> bool {}
