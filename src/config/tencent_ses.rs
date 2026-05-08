use serde::Deserialize;
use summer::config::Configurable;

#[derive(Deserialize, Configurable, Clone)]
#[config_prefix = "tencent_ses"]
pub struct TencentSes {
    pub secret_id: String,
    pub secret_key: String,
    #[serde(default = "default_region")]
    pub region: String,
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    #[serde(default)]
    pub from_email_address: String,
    #[serde(default)]
    pub unsubscribe_url: String,
}

fn default_region() -> String {
    "ap-guangzhou".to_string()
}

fn default_endpoint() -> String {
    "ses.tencentcloudapi.com".to_string()
}
