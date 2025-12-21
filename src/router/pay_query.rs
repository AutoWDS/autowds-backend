use crate::model::sea_orm_active_enums::{OrderLevel, PayFrom};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeCreateQuery {
    pub level: OrderLevel,
    pub pay_from: PayFrom,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PayStatusQuery {
    pub order_id: i32,
}