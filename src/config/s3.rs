use serde::Deserialize;
use summer::config::Configurable;

/// 与 **autowds-instance** `[s3]` 对齐，用于按 `task_instance.log_key` 从对象存储读取已归档的 NDJSON 任务日志。
#[derive(Debug, Clone, Configurable, Deserialize)]
#[config_prefix = "s3"]
pub struct TaskLogS3Config {
    #[serde(default)]
    pub endpoint: String,
    #[serde(default)]
    pub bucket: String,
    #[serde(default)]
    pub access_key_id: String,
    #[serde(default)]
    pub secret_access_key: String,
    #[serde(default = "default_region")]
    pub region: String,
    #[serde(default)]
    pub prefix: String,
}

fn default_region() -> String {
    "us-east-1".into()
}

impl TaskLogS3Config {
    pub fn is_configured(&self) -> bool {
        !self.endpoint.trim().is_empty()
            && !self.bucket.trim().is_empty()
            && !self.access_key_id.trim().is_empty()
            && !self.secret_access_key.trim().is_empty()
    }
}
