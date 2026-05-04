use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use validator::Validate;

use crate::model::scraper_task::ScraperTaskData;

/// # 任务查询请求
#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct ScraperTaskQuery {
    /// # 任务名
    #[validate(length(max = 80, message = "查询名称过长"))]
    pub name: Option<String>,
    /// # 创建时间起始
    pub start_time: Option<String>,
    /// # 创建时间结束
    pub end_time: Option<String>,
}

/// # 保存任务请求
#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct ScraperTaskReq {
    /// # 任务名
    #[validate(length(max = 80, message = "查询名称过长"))]
    pub name: String,

    /// # 任务级配置（调度 + 数据质量等，见 [`ScraperTaskData`]）
    #[serde(default)]
    pub data: Option<ScraperTaskData>,

    /// # 任务规则定义
    #[serde(default)]
    pub rule: Value,
}

/// # 更新任务请求
#[derive(Debug, Deserialize, Validate, JsonSchema)]
pub struct ScraperUpdateTaskReq {
    #[serde(default)]
    pub data: Option<ScraperTaskData>,

    #[serde(default)]
    pub rule: Value,
}
