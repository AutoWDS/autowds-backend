use actix_web::{middleware::ErrorHandlers, web, App, HttpServer};
use deadpool_redis::{Config, Pool, Runtime};
use envconfig::Envconfig;
use ormlite::postgres::{PgPool, PgPoolOptions};
use std::net::SocketAddr;

mod app;
mod middleware;
mod model;
mod utils;

#[macro_use]
extern crate lazy_static;

/// 数据库连接
#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    redis: Pool,
}

/// 服务相关配置，开发时读取.env文件，生产时读取docker容器的环境变量
#[derive(Envconfig)]
struct AppConfig {
    #[envconfig(from = "SERVER_PORT", default = "8080")]
    pub server_port: u16,

    #[envconfig(
        from = "DATABASE_URL",
        default = "postgres://postgres:password@localhost"
    )]
    pub db_url: String,

    #[envconfig(from = "POOL_SIZE", default = "3")]
    pub db_max_connection: u32,

    #[envconfig(from = "REDIS_URL", default = "redis://127.0.0.1/")]
    pub redis_url: String,

    #[envconfig(from = "DEBUG", default = "false")]
    pub debug: bool,
}

/// 入口函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init();

    let config = AppConfig::init_from_env().expect("envconfig init failed");

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    log::info!("listening on {}", addr);

    // 初始化数据库连接
    let db = PgPoolOptions::new()
        .max_connections(config.db_max_connection)
        .connect(&config.db_url)
        .await
        .expect(format!("can't connect to database: {}", config.db_url).as_str());

    // redis
    let redis = Config::from_url(&config.redis_url)
        .create_pool(Some(Runtime::Tokio1))
        .expect(format!("can't connect to redis: {}", config.redis_url).as_str());

    let app_state = AppState { db, redis };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middleware::real_ip_parser::RealIP)
            .wrap(middleware::auth_middleware::Authentication)
            .wrap(ErrorHandlers::new().default_handler(middleware::error_handler::handle_error))
            .configure(app::config)
    })
    .workers(if config.debug { 1 } else { num_cpus::get() })
    .bind(addr)?
    .run()
    .await
}
