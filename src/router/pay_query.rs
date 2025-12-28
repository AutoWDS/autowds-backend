use crate::model::sea_orm_active_enums::{OrderLevel, PayFrom};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct TradeCreateQuery {
    pub level: OrderLevel,
    pub pay_from: PayFrom,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct PayStatusQuery {
    #[validate(range(min = 1, message = "订单ID必须大于0"))]
    pub order_id: i32,
}