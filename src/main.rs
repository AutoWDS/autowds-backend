mod dto;
mod http;
mod model;
mod utils;
mod config;

use spring::{auto_config, App};
use spring_mail::MailPlugin;
use spring_redis::RedisPlugin;
use spring_sea_orm::SeaOrmPlugin;
use spring_web::{WebConfigurator, WebPlugin};

#[auto_config(WebConfigurator)]
#[tokio::main]
async fn main() {
    App::new()
        .add_plugin(WebPlugin)
        .add_plugin(SeaOrmPlugin)
        .add_plugin(MailPlugin)
        .add_plugin(RedisPlugin)
        .run()
        .await
}
