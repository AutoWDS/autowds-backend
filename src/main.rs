use dotenvy::dotenv;

mod app;
mod model;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    app::run().await
}
