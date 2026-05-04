//! 任务实例日志 **SSE** 与 **Redis Pub/Sub** 的封装。
//!
//! - [`channel`]：频道名、结束帧识别、统一的 `logs_complete` 事件。
//! - [`archive`]：S3 等处取回的 NDJSON 文本 → SSE 回放。
//! - [`redis_live`]：实时订阅实例进程 `PUBLISH` 的日志行。
//!
//! 路由层只做鉴权与数据源选择（`log_key` / Redis），具体协议在此模块内聚。

mod archive;
mod channel;
mod redis_live;

pub use archive::open_archive_ndjson_sse;
pub use channel::redis_pubsub_channel;
pub use redis_live::open_redis_pubsub_log_sse;
