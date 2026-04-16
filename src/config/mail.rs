use serde::Deserialize;
use summer::config::Configurable;

#[derive(Deserialize, Configurable, Clone)]
#[config_prefix = "email"]
pub struct Email {
    pub from: String,
    /// 对外站点/API 根 URL（邮件退订链接等），无尾部斜杠
    #[serde(default)]
    pub public_base_url: String,
}

impl Email {
    /// 去掉末尾 `/`，便于拼接路径
    pub fn base_url_trimmed(&self) -> &str {
        self.public_base_url.trim_end_matches('/')
    }
}
