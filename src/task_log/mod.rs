//! 任务实例日志：归档读取（S3 等）与 SSE 输出的统一模块。
//!
//! - `s3`：从对象存储读取实例归档 NDJSON（支持 gzip）。
//! - `sse`：归档回放与 Redis Pub/Sub 实时流的 SSE 封装。

pub mod s3;
pub mod sse;

