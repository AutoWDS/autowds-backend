use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct LeadQuery {
    pub keyword: Option<String>,
    pub source: Option<String>,
    pub unsubscribed: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ImportLeadsReq {
    #[validate(length(min = 1, max = 5_000_000, message = "CSV 内容不能为空或过大"))]
    pub csv_content: String,
    #[validate(length(max = 80, message = "来源不能超过80个字符"))]
    pub source: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ImportLeadsResp {
    pub created: usize,
    pub duplicated: usize,
    pub invalid: usize,
}

#[derive(Debug, Serialize)]
pub struct MarketingLeadResp {
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
    pub source: Option<String>,
    pub status: String,
    pub unsubscribed: bool,
    pub created_at: String,
    pub last_seen_at: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCampaignReq {
    #[validate(length(min = 1, max = 120, message = "活动名称长度必须在1-120字符之间"))]
    pub name: String,
    #[validate(length(min = 1, max = 200, message = "邮件主题长度必须在1-200字符之间"))]
    pub subject: String,
    #[validate(length(min = 1, max = 500, message = "Landing URL 长度必须在1-500字符之间"))]
    pub landing_url: String,
    #[validate(length(min = 1, max = 100, message = "腾讯云 SES 模板 ID 不能为空"))]
    pub provider_template_id: String,
    #[validate(length(min = 1, max = 10000, message = "请选择1-10000个线索"))]
    pub lead_ids: Vec<i64>,
}

#[derive(Debug, Serialize)]
pub struct MarketingCampaignResp {
    pub id: i64,
    pub name: String,
    pub subject: String,
    pub landing_url: String,
    pub status: String,
    pub provider_receiver_id: Option<String>,
    pub provider_template_id: Option<String>,
    pub provider_task_id: Option<String>,
    pub created_at: String,
    pub delivery_count: u64,
}

#[derive(Debug, Deserialize)]
pub struct CampaignQuery {
    pub keyword: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SendCampaignResp {
    pub receiver_id: i64,
    pub task_id: i64,
    pub delivery_count: u64,
}

#[derive(Debug, Deserialize)]
pub struct MarketingEventReq {
    pub event_type: String,
    pub mtk: String,
    pub url: Option<String>,
    pub meta: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct MarketingEventResp {
    pub recorded: bool,
}

#[derive(Debug, Serialize)]
pub struct FunnelMetric {
    pub event_type: String,
    pub count: usize,
    pub rate: f64,
}

#[derive(Debug, Serialize)]
pub struct CampaignFunnelResp {
    pub campaign_id: i64,
    pub sent: usize,
    pub metrics: Vec<FunnelMetric>,
}
