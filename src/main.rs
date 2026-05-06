mod config;
mod model;
mod plugin;
mod router;
mod service;
mod task;
mod utils;
mod views;

use summer::App;
use summer_apalis::{ApalisConfigurator, ApalisPlugin};
use summer_job::JobPlugin;
use summer_mail::MailPlugin;
use summer_redis::RedisPlugin;
use summer_sea_orm::SeaOrmPlugin;
use summer_sqlx::SqlxPlugin;
use summer_web::{WebConfigurator, WebPlugin};
use plugin::pay::PayPlugin;
use plugin::s3::S3Plugin;

#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(WebPlugin)
        .add_plugin(SeaOrmPlugin)
        .add_plugin(SqlxPlugin)
        .add_plugin(MailPlugin)
        .add_plugin(RedisPlugin)
        .add_plugin(JobPlugin)
        .add_plugin(ApalisPlugin)
        .add_plugin(PayPlugin)
        .add_plugin(S3Plugin)
        .add_router(router::router())
        .add_worker(task::add_storage)
        .add_scheduler(task::recover_task_schedules)
        .run()
        .await
}
