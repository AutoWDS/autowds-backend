use crate::model::sea_orm_active_enums::ProductEdition;
use schemars::JsonSchema;
use serde::Serialize;

/// # 用户Token
#[derive(Debug, Serialize, JsonSchema)]
pub struct UserToken {
    pub(crate) id: i64,
    pub(crate) is_admin: bool,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) edition: ProductEdition,
    pub(crate) token: String,
}
