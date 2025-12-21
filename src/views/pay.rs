use crate::model::sea_orm_active_enums::{OrderLevel, PayFrom};
use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GlobalVariables {
    pub app_name: String,
    pub version: String,
}

impl Default for GlobalVariables {
    fn default() -> Self {
        Self {
            app_name: "AutoWDS".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

#[derive(Template)]
#[template(path = "pay/create.html")]
pub struct PayTradeCreateTemplate {
    pub global: GlobalVariables,
    pub user_id: i32,
}

#[derive(Template)]
#[template(path = "pay/redirect.html")]
pub struct PayRedirectTemplate {
    pub global: GlobalVariables,
    pub order_id: i32,
    pub qrcode_url: String,
    pub pay_from: PayFrom,
}

#[derive(Debug, Serialize)]
pub struct PayStatusResponse {
    pub order_id: i32,
    pub status: String,
    pub level: OrderLevel,
    pub pay_from: PayFrom,
    pub created: String,
    pub confirm: Option<String>,
}