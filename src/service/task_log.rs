use std::{
    collections::HashMap,
    convert::Infallible,
    io::{Cursor, Read},
    sync::LazyLock,
};

use anyhow::Context;
use flate2::read::GzDecoder;
use sea_orm::{DbConn, EntityTrait};
use serde_json::Value;
use summer::{plugin::service::Service, tracing};
use summer_redis::{config::RedisConfig, redis};
use summer_web::axum::response::sse::{Event, Sse};
use summer_web::error::{KnownWebError, Result};
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;

use crate::model::prelude::{ScraperTask, TaskInstance};
use crate::model::task_instance;
use crate::plugin::s3::S3Client;

pub type TaskLogSse = Sse<ReceiverStream<std::result::Result<Event, Infallible>>>;

const MAX_ARCHIVED_LOG_BYTES: usize = 32 * 1024 * 1024;
const ARCHIVE_SSE_CHANNEL_CAPACITY: usize = 64;
const LIVE_SSE_CHANNEL_CAPACITY: usize = 100;
const LIVE_STREAM_BROADCAST_CAPACITY: usize = 512;
const LIVE_STREAM_BLOCK_MS: u64 = 5_000;
const LIVE_STREAM_BATCH_SIZE: usize = 100;
const SSE_EVENT_LOGS_COMPLETE: &str = "logs_complete";
const SSE_DATA_LOGS_COMPLETE: &str = "{}";
const REDIS_STREAM_LOG_PAYLOAD_FIELD: &str = "payload";

static LIVE_LOG_STREAMS: LazyLock<Mutex<HashMap<String, broadcast::Sender<LiveLogEvent>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug)]
enum LiveLogEvent {
    Message(String),
    Complete,
}

#[derive(Clone, Service)]
pub struct TaskLogService {
    #[inject(component)]
    pub db: DbConn,
    #[inject(config)]
    redis_config: RedisConfig,
    #[inject(component)]
    s3_client: S3Client,
}

impl TaskLogService {
    pub async fn open_instance_logs(
        &self,
        user_id: i64,
        task_id: i64,
        instance_id: i64,
    ) -> Result<TaskLogSse> {
        let inst = self
            .find_checked_instance(user_id, task_id, instance_id)
            .await?;

        if let Some(log_key) = Self::archived_log_key(&inst) {
            return self.open_archived_log(log_key).await;
        }

        Ok(self.open_live_log(task_id, instance_id).await)
    }

    async fn find_checked_instance(
        &self,
        user_id: i64,
        task_id: i64,
        instance_id: i64,
    ) -> Result<task_instance::Model> {
        ScraperTask::find_check_task(&self.db, task_id, user_id).await?;

        let inst = TaskInstance::find_by_id(instance_id)
            .one(&self.db)
            .await
            .context("查询 task_instance 失败")?
            .ok_or_else(|| KnownWebError::not_found("实例不存在"))?;
        if inst.task_id != task_id {
            return Err(KnownWebError::bad_request("实例与任务不匹配"))?;
        }

        Ok(inst)
    }

    fn archived_log_key(inst: &task_instance::Model) -> Option<&str> {
        inst.log_key
            .as_deref()
            .map(str::trim)
            .filter(|key| !key.is_empty())
    }

    async fn open_archived_log(&self, log_key: &str) -> Result<TaskLogSse> {
        if !self.s3_client.is_configured() {
            return Err(KnownWebError::internal_server_error(
                "实例已写入归档键 log_key，但服务端 [s3] 未配置完整，无法从对象存储读取日志",
            ))?;
        }

        let bytes = self
            .s3_client
            .get_object_bytes(log_key)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, %log_key, "S3 GetObject 任务归档日志失败");
                KnownWebError::internal_server_error(format!("读取归档日志失败：{e}"))
            })?;
        let body = Self::decode_archived_task_log_ndjson(log_key, &bytes).map_err(|e| {
            tracing::error!(error = %e, %log_key, "归档任务日志解码失败");
            KnownWebError::internal_server_error(format!("解码归档日志失败：{e}"))
        })?;

        Ok(Self::open_archive_ndjson_sse(body))
    }

    async fn open_live_log(&self, task_id: i64, instance_id: i64) -> TaskLogSse {
        let stream_key = Self::redis_stream_key(task_id, instance_id);
        let receiver =
            Self::subscribe_live_log_stream(self.redis_config.uri.clone(), stream_key).await;
        Self::open_redis_stream_log_sse(receiver)
    }

    fn check_archived_log_size(bytes: &[u8]) -> anyhow::Result<()> {
        if bytes.len() > MAX_ARCHIVED_LOG_BYTES {
            anyhow::bail!(
                "归档日志过大（{} 字节，上限 {}）",
                bytes.len(),
                MAX_ARCHIVED_LOG_BYTES
            );
        }
        Ok(())
    }

    /// 将 GetObject 得到的字节解码为 NDJSON 文本：`.gz` 键或 gzip 魔数则先 gunzip。
    fn decode_archived_task_log_ndjson(log_key: &str, bytes: &[u8]) -> anyhow::Result<String> {
        Self::check_archived_log_size(bytes)?;
        let gzip_by_key = log_key.trim_end().ends_with(".gz");
        if gzip_by_key || Self::is_gzip_payload(bytes) {
            let mut decoder = GzDecoder::new(Cursor::new(bytes));
            let mut out = String::new();
            decoder
                .read_to_string(&mut out)
                .context("gzip 解压归档任务日志失败（内容损坏或非 gzip）")?;
            return Ok(out);
        }
        Ok(String::from_utf8_lossy(bytes).into_owned())
    }

    fn is_gzip_payload(bytes: &[u8]) -> bool {
        bytes.len() >= 2 && bytes[0] == 0x1f && bytes[1] == 0x8b
    }

    /// 将整段 NDJSON 文本推成 SSE；连接在发完 `logs_complete` 后由发送端关闭。
    fn open_archive_ndjson_sse(ndjson_text: String) -> TaskLogSse {
        let (tx, rx) = mpsc::channel(ARCHIVE_SSE_CHANNEL_CAPACITY);
        tokio::spawn(async move {
            for line in ndjson_text.lines() {
                let t = line.trim_end();
                if t.is_empty() {
                    continue;
                }
                if tx.send(Ok(Event::default().data(t))).await.is_err() {
                    return;
                }
            }
            let _ = tx.send(Ok(Self::sse_event_logs_complete())).await;
        });
        Sse::new(ReceiverStream::new(rx))
    }

    /// 为避免每个浏览器 SSE 都建立 Redis 阻塞读连接，同一个 Redis Stream 在当前后端进程内只创建一个 reader。
    async fn subscribe_live_log_stream(
        redis_uri: String,
        stream_key: String,
    ) -> broadcast::Receiver<LiveLogEvent> {
        let mut streams = LIVE_LOG_STREAMS.lock().await;
        if let Some(sender) = streams.get(&stream_key) {
            return sender.subscribe();
        }

        let (sender, receiver) = broadcast::channel(LIVE_STREAM_BROADCAST_CAPACITY);
        streams.insert(stream_key.clone(), sender.clone());
        tokio::spawn(Self::run_redis_stream_log_fanout(
            redis_uri, stream_key, sender,
        ));
        receiver
    }

    async fn run_redis_stream_log_fanout(
        redis_uri: String,
        stream_key: String,
        sender: broadcast::Sender<LiveLogEvent>,
    ) {
        let client = match redis::Client::open(redis_uri.as_str()) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Redis client 创建失败: {e}");
                Self::remove_live_log_stream(&stream_key, &sender).await;
                return;
            }
        };
        let mut conn = match client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(e) => {
                tracing::error!("Redis Stream 读取连接失败: {e}");
                Self::remove_live_log_stream(&stream_key, &sender).await;
                return;
            }
        };
        let mut last_id = "0-0".to_string();

        loop {
            if sender.receiver_count() == 0
                && Self::remove_live_log_stream_if_unused(&stream_key, &sender).await
            {
                return;
            }

            let read_result = redis::cmd("XREAD")
                .arg("BLOCK")
                .arg(LIVE_STREAM_BLOCK_MS)
                .arg("COUNT")
                .arg(LIVE_STREAM_BATCH_SIZE)
                .arg("STREAMS")
                .arg(&stream_key)
                .arg(&last_id)
                .query_async::<redis::Value>(&mut conn)
                .await;

            let value = match read_result {
                Ok(value) => value,
                Err(e) => {
                    tracing::error!(error = %e, %stream_key, "Redis Stream XREAD 任务日志失败");
                    Self::remove_live_log_stream(&stream_key, &sender).await;
                    break;
                }
            };

            for entry in Self::parse_xread_payloads(&value) {
                last_id = entry.id;
                if let Some(payload) = entry.payload {
                    if Self::is_stream_eof_payload(&payload) {
                        let _ = sender.send(LiveLogEvent::Complete);
                        Self::remove_live_log_stream(&stream_key, &sender).await;
                        return;
                    }
                    if sender.send(LiveLogEvent::Message(payload)).is_err()
                        && sender.receiver_count() == 0
                        && Self::remove_live_log_stream_if_unused(&stream_key, &sender).await
                    {
                        return;
                    }
                }
            }
        }
    }

    async fn remove_live_log_stream(stream_key: &str, sender: &broadcast::Sender<LiveLogEvent>) {
        let mut streams = LIVE_LOG_STREAMS.lock().await;
        if streams
            .get(stream_key)
            .is_some_and(|current| current.same_channel(sender))
        {
            streams.remove(stream_key);
        }
    }

    async fn remove_live_log_stream_if_unused(
        stream_key: &str,
        sender: &broadcast::Sender<LiveLogEvent>,
    ) -> bool {
        let mut streams = LIVE_LOG_STREAMS.lock().await;
        let should_remove = streams
            .get(stream_key)
            .is_some_and(|current| current.same_channel(sender) && current.receiver_count() == 0);
        if should_remove {
            streams.remove(stream_key);
        }
        should_remove
    }

    fn open_redis_stream_log_sse(mut receiver: broadcast::Receiver<LiveLogEvent>) -> TaskLogSse {
        let (tx, rx) = mpsc::channel(LIVE_SSE_CHANNEL_CAPACITY);
        tokio::spawn(async move {
            loop {
                match receiver.recv().await {
                    Ok(LiveLogEvent::Message(payload)) => {
                        if tx.send(Ok(Event::default().data(payload))).await.is_err() {
                            break;
                        }
                    }
                    Ok(LiveLogEvent::Complete) => {
                        let _ = tx.send(Ok(Self::sse_event_logs_complete())).await;
                        break;
                    }
                    Err(broadcast::error::RecvError::Lagged(skipped)) => {
                        tracing::warn!(skipped, "任务日志 SSE 客户端消费过慢，已跳过部分实时日志");
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        });
        Sse::new(ReceiverStream::new(rx))
    }

    /// 与实例侧 `task_logger` / `LogManager::publish_logs_stream_eof` 使用同一 Stream key 模板。
    fn redis_stream_key(task_id: i64, instance_id: i64) -> String {
        format!("task:{task_id}:{instance_id}:logs")
    }

    fn parse_xread_payloads(value: &redis::Value) -> Vec<StreamLogEntry> {
        let redis::Value::Array(streams) = value else {
            return Vec::new();
        };
        streams.iter().flat_map(Self::parse_xread_stream).collect()
    }

    fn parse_xread_stream(value: &redis::Value) -> Vec<StreamLogEntry> {
        let redis::Value::Array(parts) = value else {
            return Vec::new();
        };
        let Some(redis::Value::Array(entries)) = parts.get(1) else {
            return Vec::new();
        };
        entries.iter().filter_map(Self::parse_xread_entry).collect()
    }

    fn parse_xread_entry(value: &redis::Value) -> Option<StreamLogEntry> {
        let redis::Value::Array(parts) = value else {
            return None;
        };
        let id = Self::redis_value_to_string(parts.first()?)?;
        let payload = Self::parse_stream_payload_field(parts.get(1));
        Some(StreamLogEntry { id, payload })
    }

    fn parse_stream_payload_field(fields: Option<&redis::Value>) -> Option<String> {
        match fields? {
            redis::Value::Array(items) => items.chunks_exact(2).find_map(|pair| {
                let field = Self::redis_value_to_string(&pair[0])?;
                if field == REDIS_STREAM_LOG_PAYLOAD_FIELD {
                    Self::redis_value_to_string(&pair[1])
                } else {
                    None
                }
            }),
            redis::Value::Map(items) => items.iter().find_map(|(field, value)| {
                if Self::redis_value_to_string(field)? == REDIS_STREAM_LOG_PAYLOAD_FIELD {
                    Self::redis_value_to_string(value)
                } else {
                    None
                }
            }),
            _ => None,
        }
    }

    fn redis_value_to_string(value: &redis::Value) -> Option<String> {
        match value {
            redis::Value::BulkString(bytes) => String::from_utf8(bytes.clone()).ok(),
            redis::Value::SimpleString(s) => Some(s.clone()),
            _ => None,
        }
    }

    /// 判断 Redis 消息是否为「日志流结束」控制帧（非业务 `LogEntry`）。
    fn is_stream_eof_payload(payload: &str) -> bool {
        serde_json::from_str::<Value>(payload)
            .ok()
            .and_then(|v| v.get("__autowds_logs_eof").and_then(|x| x.as_bool()))
            == Some(true)
    }

    fn sse_event_logs_complete() -> Event {
        Event::default()
            .event(SSE_EVENT_LOGS_COMPLETE)
            .data(SSE_DATA_LOGS_COMPLETE)
    }
}

struct StreamLogEntry {
    id: String,
    payload: Option<String>,
}
