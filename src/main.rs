use actix_web::{middleware::ErrorHandlers, web, App, HttpServer};
use deadpool_redis::{Config, Pool, Runtime};
use envconfig::Envconfig;
use rbatis::RBatis;
use rbdc_pg::driver::PgDriver;
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
    rbatis: RBatis,
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
    pub db_max_connection: usize,

    #[envconfig(from = "REDIS_URL", default = "redis://127.0.0.1/")]
    pub redis_url: String,
}

/// 入口函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::init();

    let config = AppConfig::init_from_env().unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    log::info!("listening on {}", addr);

    // 初始化数据库连接
    let rb = RBatis::new();
    rb.link(PgDriver {}, &config.db_url)
        .await
        .expect(format!("can't connect to database: {}", config.db_url).as_str());
    rb.get_pool().unwrap().resize(config.db_max_connection);

    let app_state = AppState {
        rbatis: rb,
        redis: Config::from_url(&config.redis_url)
            .create_pool(Some(Runtime::Tokio1))
            .unwrap(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middleware::real_ip_parser::RealIP)
            .wrap(middleware::auth_middleware::Authentication)
            .wrap(ErrorHandlers::new().default_handler(middleware::error_handler::handle_error))
            .configure(app::config)
    })
    .workers(num_cpus::get())
    .bind(addr)?
    .run()
    .await
}
