use serde::Deserialize;
use summer::config::Configurable;

/// Apalis Redis 存储的队列命名空间（传给 `apalis_redis::RedisConfig::new`）。
/// 须与 **autowds-instance** 守护进程里的 `[redis] queue` / `redis_queue` 一致。
#[derive(Debug, Clone, Configurable, Deserialize)]
#[config_prefix = "apalis"]
pub struct ApalisConfig {
    #[serde(default = "default_queue")]
    pub queue: String,
}

fn default_queue() -> String {
    // 与 `RedisStorage::<i64>::new(conn)` 内部默认（`type_name::<i64>()`）一致，避免已有 Redis 键迁移
    "i64".into()
}
