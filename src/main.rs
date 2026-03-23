mod config;
mod model;
mod router;
mod task;
mod utils;
mod views;

use summer::App;
use summer_apalis::{ApalisConfigurator, ApalisPlugin};
use summer_job::JobPlugin;
use summer_mail::MailPlugin;
use summer_redis::RedisPlugin;
use summer_sea_orm::SeaOrmPlugin;
use summer_web::{WebConfigurator, WebPlugin};
use utils::pay_plugin::PayPlugin;

#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(WebPlugin)
        .add_plugin(SeaOrmPlugin)
        .add_plugin(MailPlugin)
        .add_plugin(RedisPlugin)
        .add_plugin(JobPlugin)
        .add_plugin(ApalisPlugin)
        .add_plugin(PayPlugin)
        .add_router(router::router())
        .add_worker(task::add_storage)
        .run()
        .await
}
