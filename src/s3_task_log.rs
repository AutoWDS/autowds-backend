//! 从 S3 兼容存储读取实例归档的 NDJSON 任务日志（与 autowds-instance `S3Archiver` 的 key 规则一致）。

use anyhow::Context as _;
use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use aws_sdk_s3::config::{Builder as S3ConfigBuilder, Region};
use aws_sdk_s3::Client;

use crate::config::s3::TaskLogS3Config;

/// 单文件上限，防止 OOM。
const MAX_ARCHIVED_LOG_BYTES: usize = 32 * 1024 * 1024;

/// `log_key` 为库中存的相对键（如 `tasks/{taskId}/{instanceId}.jsonl`），与上传时传入 `upload` 的 `remote_key` 一致；若配置了 `prefix`，与实例侧相同拼到对象完整 key。
pub fn object_key_for_log(cfg: &TaskLogS3Config, log_key: &str) -> String {
    let log_key = log_key.trim().trim_start_matches('/');
    let p = cfg.prefix.trim();
    if p.is_empty() {
        log_key.to_string()
    } else {
        format!("{}/{}", p.trim_end_matches('/'), log_key)
    }
}

pub async fn fetch_archived_task_log_bytes(cfg: &TaskLogS3Config, log_key: &str) -> anyhow::Result<Vec<u8>> {
    let key = object_key_for_log(cfg, log_key);

    let creds = Credentials::new(
        cfg.access_key_id.trim(),
        cfg.secret_access_key.trim(),
        None,
        None,
        "autowds-backend-task-log",
    );

    let sdk = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(cfg.region.trim().to_string()))
        .credentials_provider(creds)
        .endpoint_url(cfg.endpoint.trim())
        .load()
        .await;

    let s3_conf = S3ConfigBuilder::from(&sdk).force_path_style(true).build();
    let client = Client::from_conf(s3_conf);

    let resp = client
        .get_object()
        .bucket(cfg.bucket.trim())
        .key(&key)
        .send()
        .await
        .with_context(|| format!("S3 GetObject bucket={} key={}", cfg.bucket.trim(), key))?;

    let body = resp
        .body
        .collect()
        .await
        .with_context(|| format!("读取 S3 对象 body key={}", key))?;

    let bytes = body.into_bytes();
    if bytes.len() > MAX_ARCHIVED_LOG_BYTES {
        anyhow::bail!(
            "归档日志过大（{} 字节，上限 {}）",
            bytes.len(),
            MAX_ARCHIVED_LOG_BYTES
        );
    }
    Ok(bytes.to_vec())
}
