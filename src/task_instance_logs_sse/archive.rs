//! 已归档 NDJSON（S3 GetObject 结果）→ SSE：逐行 `message`，最后 `logs_complete`。

use std::convert::Infallible;

use summer_web::axum::response::sse::{Event, Sse};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use super::channel::sse_event_logs_complete;

const ARCHIVE_SSE_CHANNEL_CAPACITY: usize = 64;

/// 将整段 NDJSON 文本推成 SSE；连接在发完 `logs_complete` 后由发送端关闭。
pub fn open_archive_ndjson_sse(ndjson_text: String) -> Sse<ReceiverStream<Result<Event, Infallible>>> {
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
        let _ = tx.send(Ok(sse_event_logs_complete())).await;
    });
    Sse::new(ReceiverStream::new(rx))
}
