mod config;
mod model;
mod router;
mod task;
mod utils;
mod views;

use spring::App;
use spring_apalis::{ApalisConfigurator, ApalisPlugin};
use spring_job::JobPlugin;
use spring_mail::MailPlugin;
use spring_redis::RedisPlugin;
use spring_sea_orm::SeaOrmPlugin;
use spring_web::{WebConfigurator, WebPlugin};
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
