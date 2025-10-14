use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use validator::Validate;

use crate::model::scraper_task::ScheduleData;

/// # 任务查询请求
#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct ScraperTaskQuery {
    /// # 任务名
    #[validate(length(max = 80, message = "查询名称过长"))]
    pub name: Option<String>,
}

/// # 保存任务请求
#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct ScraperTaskReq {
    /// # 任务名
    #[validate(length(max = 80, message = "查询名称过长"))]
    pub name: String,

    /// # 数据
    #[serde(default)]
    pub data: Option<ScheduleData>,

    /// # 任务规则定义
    #[serde(default)]
    pub rule: Value,
}

/// # 更新任务请求
#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct ScraperUpdateTaskReq {
    #[serde(default)]
    pub data: Option<ScheduleData>,

    #[serde(default)]
    pub rule: Value,
}
