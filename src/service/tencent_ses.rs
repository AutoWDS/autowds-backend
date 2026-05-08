use crate::config::tencent_ses::TencentSes;
use anyhow::{anyhow, Context};
use chrono::Utc;
use hmac::{Hmac, KeyInit, Mac};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct TencentSesClient {
    config: TencentSes,
    http: Client,
}

#[derive(Debug, Deserialize)]
pub struct TencentResponse<T> {
    #[serde(rename = "Response")]
    pub response: TencentResponseBody<T>,
}

#[derive(Debug, Deserialize)]
pub struct TencentResponseBody<T> {
    #[serde(flatten)]
    pub data: Option<T>,
    #[serde(rename = "Error")]
    pub error: Option<TencentError>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Debug, Deserialize)]
pub struct TencentError {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateReceiverResp {
    #[serde(rename = "ReceiverId")]
    pub receiver_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct BatchSendEmailResp {
    #[serde(rename = "TaskId")]
    pub task_id: i64,
}

#[derive(Debug, Serialize)]
pub struct ReceiverDetailWithData {
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "TemplateData")]
    pub template_data: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SendTaskSummary {
    #[serde(rename = "TaskId")]
    pub task_id: Option<i64>,
    #[serde(rename = "TaskStatus")]
    pub task_status: Option<i64>,
    #[serde(rename = "ReceiverId")]
    pub receiver_id: Option<i64>,
    #[serde(rename = "RequestCount")]
    pub request_count: Option<i64>,
    #[serde(rename = "AcceptedCount")]
    pub accepted_count: Option<i64>,
    #[serde(rename = "DeliveredCount")]
    pub delivered_count: Option<i64>,
    #[serde(rename = "OpenedCount")]
    pub opened_count: Option<i64>,
    #[serde(rename = "ClickedCount")]
    pub clicked_count: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct ListSendTasksResp {
    #[serde(rename = "Data")]
    data: Option<Vec<SendTaskSummary>>,
}

impl TencentSesClient {
    pub fn new(config: TencentSes) -> Self {
        Self {
            config,
            http: Client::new(),
        }
    }

    pub async fn create_receiver(&self, name: &str, desc: &str) -> anyhow::Result<i64> {
        let payload = json!({
            "ReceiversName": name,
            "Desc": desc,
        });
        let resp: CreateReceiverResp = self.request("CreateReceiver", payload).await?;
        Ok(resp.receiver_id)
    }

    pub async fn create_receiver_detail_with_data(
        &self,
        receiver_id: i64,
        details: Vec<ReceiverDetailWithData>,
    ) -> anyhow::Result<()> {
        let payload = json!({
            "ReceiverId": receiver_id,
            "Datas": details,
        });
        let _: Value = self
            .request("CreateReceiverDetailWithData", payload)
            .await
            .context("create receiver detail failed")?;
        Ok(())
    }

    pub async fn batch_send_email(
        &self,
        receiver_id: i64,
        template_id: i64,
        subject: &str,
    ) -> anyhow::Result<i64> {
        let payload = json!({
            "FromEmailAddress": self.config.from_email_address,
            "ReceiverId": receiver_id,
            "Template": {
                "TemplateID": template_id,
                "TemplateData": "{}",
            },
            "Subject": subject,
            "Unsubscribe": self.config.unsubscribe_url,
        });
        let resp: BatchSendEmailResp = self.request("BatchSendEmail", payload).await?;
        Ok(resp.task_id)
    }

    pub async fn list_send_tasks(&self, task_id: i64) -> anyhow::Result<Vec<SendTaskSummary>> {
        let payload = json!({ "Status": 10, "TaskId": task_id, "Offset": 0, "Limit": 10 });
        let resp: ListSendTasksResp = self.request("ListSendTasks", payload).await?;
        Ok(resp.data.unwrap_or_default())
    }

    async fn request<T: DeserializeOwned>(
        &self,
        action: &str,
        payload: Value,
    ) -> anyhow::Result<T> {
        if self.config.secret_id.is_empty() || self.config.secret_key.is_empty() {
            return Err(anyhow!("腾讯云 SES SecretId/SecretKey 未配置"));
        }
        if self.config.from_email_address.is_empty() {
            return Err(anyhow!("腾讯云 SES 发信地址未配置"));
        }

        let timestamp = Utc::now().timestamp();
        let body =
            serde_json::to_string(&payload).context("serialize tencent ses payload failed")?;
        let authorization = self.sign(action, timestamp, &body)?;
        let url = format!("https://{}", self.config.endpoint);
        let response = self
            .http
            .post(url)
            .header("Authorization", authorization)
            .header("Content-Type", "application/json; charset=utf-8")
            .header("Host", &self.config.endpoint)
            .header("X-TC-Action", action)
            .header("X-TC-Timestamp", timestamp.to_string())
            .header("X-TC-Version", "2020-10-02")
            .header("X-TC-Region", &self.config.region)
            .body(body)
            .send()
            .await
            .with_context(|| format!("request tencent ses {action} failed"))?;

        let status = response.status();
        let text = response
            .text()
            .await
            .context("read tencent ses body failed")?;
        if !status.is_success() {
            return Err(anyhow!("腾讯云 SES {action} HTTP {status}: {text}"));
        }
        let envelope: TencentResponse<T> = serde_json::from_str(&text)
            .with_context(|| format!("parse tencent ses {action} failed: {text}"))?;
        if let Some(error) = envelope.response.error {
            return Err(anyhow!(
                "腾讯云 SES {action} 失败 {}: {} ({})",
                error.code,
                error.message,
                envelope.response.request_id
            ));
        }
        envelope.response.data.ok_or_else(|| {
            anyhow!(
                "腾讯云 SES {action} 返回为空 ({})",
                envelope.response.request_id
            )
        })
    }

    fn sign(&self, _action: &str, timestamp: i64, body: &str) -> anyhow::Result<String> {
        let service = "ses";
        let date = chrono::DateTime::from_timestamp(timestamp, 0)
            .ok_or_else(|| anyhow!("invalid timestamp"))?
            .format("%Y-%m-%d")
            .to_string();
        let hashed_payload = hex_sha256(body.as_bytes());
        let canonical_request = format!(
            "POST\n/\n\ncontent-type:application/json; charset=utf-8\nhost:{}\n\ncontent-type;host\n{}",
            self.config.endpoint, hashed_payload
        );
        let credential_scope = format!("{date}/{service}/tc3_request");
        let string_to_sign = format!(
            "TC3-HMAC-SHA256\n{timestamp}\n{credential_scope}\n{}",
            hex_sha256(canonical_request.as_bytes())
        );
        let secret_date = hmac_sha256(
            format!("TC3{}", self.config.secret_key).as_bytes(),
            date.as_bytes(),
        )?;
        let secret_service = hmac_sha256(&secret_date, service.as_bytes())?;
        let secret_signing = hmac_sha256(&secret_service, b"tc3_request")?;
        let signature = hex::encode(hmac_sha256(&secret_signing, string_to_sign.as_bytes())?);
        Ok(format!(
            "TC3-HMAC-SHA256 Credential={}/{}, SignedHeaders=content-type;host, Signature={}",
            self.config.secret_id, credential_scope, signature
        ))
    }
}

fn hex_sha256(input: &[u8]) -> String {
    hex::encode(Sha256::digest(input))
}

fn hmac_sha256(key: &[u8], input: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut mac = HmacSha256::new_from_slice(key).context("hmac key invalid")?;
    mac.update(input);
    Ok(mac.finalize().into_bytes().to_vec())
}
