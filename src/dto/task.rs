use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ScraperTaskQuery {
    pub name: String,
}
