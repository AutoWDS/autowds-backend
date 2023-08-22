use actix_web::{web, App, HttpServer};
use envconfig::Envconfig;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{net::SocketAddr, time::Duration};

mod template;
mod user;

#[derive(Clone)]
pub struct AppState {
    pg_pool: Pool<Postgres>,
}

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "SERVER_PORT", default = "8080")]
    pub server_port: u16,

    #[envconfig(from = "DATABASE_URL", default = "postgres://postgres:password@localhost")]
    pub db_url: String,

    #[envconfig(from = "POOL_SIZE", default = "3")]
    pub db_max_connection: u32,
}

pub async fn run() -> std::io::Result<()> {
    let config = Config::init_from_env().unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    log::info!("listening on {}", addr);

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(config.db_max_connection)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.db_url)
        .await
        .expect("can't connect to database");

    let app_state = AppState { pg_pool: pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(user::user_scope())
            .service(template::template_scope())
    })
    .workers(4)
    .bind(addr)?
    .run()
    .await
}
