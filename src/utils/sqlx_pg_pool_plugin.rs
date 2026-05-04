//! 注册 [`sqlx::PgPool`]，供按 `user_id` 分表等无法走 SeaORM 映射的 SQL 使用。

use crate::config::sqlx::SqlxConfig;
use anyhow::Context as _;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use summer::{
    app::AppBuilder,
    async_trait,
    config::ConfigRegistry as _,
    plugin::{MutableComponentRegistry as _, Plugin},
};

pub struct SqlxPgPoolPlugin;

#[async_trait]
impl Plugin for SqlxPgPoolPlugin {
    async fn build(&self, app: &mut AppBuilder) {
        let cfg = app
            .get_config::<SqlxConfig>()
            .expect("读取 [sqlx] 配置失败（config/app.toml 中需有 [sqlx].uri）");
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(&cfg.uri)
            .await
            .with_context(|| format!("sqlx 连接 PostgreSQL: {}", cfg.uri))
            .expect("sqlx PgPool 初始化失败");
        app.add_component(pool);
    }
}
