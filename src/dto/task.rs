use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ScraperTaskQuery {
    #[validate(length(max = 60, message = "查询名称过长"))]
    pub name: Option<String>,
}
