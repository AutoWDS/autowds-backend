//! Redis 频道命名与 **流结束** 载荷约定（与 autowds-instance `logging::LOG_STREAM_EOF_JSON` 对齐）。

use serde_json::Value;
use summer_web::axum::response::sse::Event;

/// SSE：归档回放与实时流结束时发往浏览器，与前端 `EventSource` 监听名一致。
pub const SSE_EVENT_LOGS_COMPLETE: &str = "logs_complete";

pub const SSE_DATA_LOGS_COMPLETE: &str = "{}";

/// 与实例侧 `task_logger` / `LogManager::publish_logs_stream_eof` 使用同一频道模板。
pub fn redis_pubsub_channel(task_id: i64, instance_id: i64) -> String {
    format!("task:{task_id}:{instance_id}:logs")
}

/// 判断 Redis 消息是否为「日志流结束」控制帧（非业务 `LogEntry`）。
pub fn is_stream_eof_payload(payload: &str) -> bool {
    serde_json::from_str::<Value>(payload)
        .ok()
        .and_then(|v| v.get("__autowds_logs_eof").and_then(|x| x.as_bool()))
        == Some(true)
}

pub fn sse_event_logs_complete() -> Event {
    Event::default()
        .event(SSE_EVENT_LOGS_COMPLETE)
        .data(SSE_DATA_LOGS_COMPLETE)
}
