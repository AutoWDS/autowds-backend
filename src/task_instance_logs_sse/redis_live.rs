//! Redis Pub/Sub → SSE：订阅实例日志频道，识别流结束帧并转发 `logs_complete`。

use std::convert::Infallible;

use futures_util::StreamExt;
use summer_redis::redis;
use summer_web::axum::response::sse::{Event, Sse};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use super::channel::{is_stream_eof_payload, sse_event_logs_complete};

const LIVE_SSE_CHANNEL_CAPACITY: usize = 100;

const CLIENT_GONE_POLL_SECS: u64 = 5;

/// 订阅 `channel`，将每条 `PUBLISH` 载荷作为默认 SSE `message` 下发；收到 EOF 控制帧则发 `logs_complete` 并退出。
pub fn open_redis_pubsub_log_sse(redis_uri: String, channel: String) -> Sse<ReceiverStream<Result<Event, Infallible>>> {
    let (tx, rx) = mpsc::channel(LIVE_SSE_CHANNEL_CAPACITY);
    tokio::spawn(async move {
        let client = match redis::Client::open(redis_uri.as_str()) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Redis client 创建失败: {e}");
                return;
            }
        };
        let mut pubsub = match client.get_async_pubsub().await {
            Ok(ps) => ps,
            Err(e) => {
                tracing::error!("Redis pub/sub 连接失败: {e}");
                return;
            }
        };
        if let Err(e) = pubsub.subscribe(&channel).await {
            tracing::error!(%channel, "Redis 订阅失败: {e}");
            return;
        }

        let mut msg_stream = pubsub.on_message();
        let mut check_interval = tokio::time::interval(tokio::time::Duration::from_secs(CLIENT_GONE_POLL_SECS));

        loop {
            tokio::select! {
                Some(msg) = msg_stream.next() => {
                    let payload: String = msg.get_payload::<String>().unwrap_or_default();
                    if is_stream_eof_payload(&payload) {
                        let _ = tx.send(Ok(sse_event_logs_complete())).await;
                        break;
                    }
                    if tx.send(Ok(Event::default().data(payload))).await.is_err() {
                        break;
                    }
                }
                _ = check_interval.tick() => {
                    if tx.is_closed() {
                        break;
                    }
                }
            }
        }
    });
    Sse::new(ReceiverStream::new(rx))
}
