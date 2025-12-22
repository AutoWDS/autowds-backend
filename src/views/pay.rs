use crate::model::sea_orm_active_enums::{OrderLevel, PayFrom};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PayStatusResponse {
    pub order_id: i32,
    pub status: String,
    pub level: OrderLevel,
    pub pay_from: PayFrom,
    pub created: String,
    pub confirm: Option<String>,
}
