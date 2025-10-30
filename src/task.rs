use spring::{
    app::AppBuilder,
    plugin::{ComponentRegistry, MutableComponentRegistry as _},
};
use spring_apalis::apalis::prelude::Monitor;
use spring_apalis::apalis::prelude::*;
use spring_apalis::apalis_redis::RedisStorage;
use spring_job::extractor::{Component, Data};
use spring_redis::Redis;

pub type TaskPublisher = RedisStorage<i64>;

pub fn add_storage(app: &mut AppBuilder, monitor: Monitor) -> Monitor {
    let redis = app.get_expect_component::<Redis>();
    app.add_component(TaskPublisher::new(redis));
    monitor
}

pub async fn dispatch_task(
    Component(mut publisher): Component<TaskPublisher>,
    Data(task_id): Data<i64>,
) {
    match publisher.push(task_id).await {
        Ok(r) => {
            tracing::info!("dispatch task success: {r:?}")
        }
        Err(e) => {
            tracing::error!("publish to redis failed: {e:?}")
        }
    }
}
