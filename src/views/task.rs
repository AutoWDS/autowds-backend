use serde::Deserialize;
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ScraperTaskQuery {
    #[validate(length(max = 80, message = "查询名称过长"))]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ScraperTaskReq {
    #[validate(length(max = 80, message = "查询名称过长"))]
    pub name: String,

    #[serde(default)]
    pub data: Value,

    #[serde(default)]
    pub rule: Value,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ScraperUpdateTaskReq {
    #[serde(default)]
    pub data: Value,

    #[serde(default)]
    pub rule: Value,
}
