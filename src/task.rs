use apalis_board_api::{
    framework::{ApiBuilder, RegisterRoute as _},
    sse::{TracingBroadcaster, TracingSubscriber},
    ui::ServeUI,
};
use spring::{
    app::AppBuilder,
    plugin::{ComponentRegistry, MutableComponentRegistry as _},
};
use spring_apalis::apalis::prelude::Monitor;
use spring_apalis::apalis::prelude::*;
use spring_apalis::apalis_redis::RedisStorage;
use spring_job::extractor::{Component, Data};
use spring_redis::Redis;

mod pay_check;

pub type TaskPublisher = RedisStorage<i64>;

pub fn add_storage(app: &mut AppBuilder, monitor: Monitor) -> Monitor {
    let broadcaster = TracingBroadcaster::create();
    let line_subscriber = TracingSubscriber::new(&broadcaster);
    app.add_layer(line_subscriber.layer());
    let redis = app.get_expect_component::<Redis>();
    let storage = TaskPublisher::new(redis);
    app.add_component(storage.clone());
    // let apalis_api = ApiBuilder::new(Router::new()).register(storage).build();
    // let router = Router::new()
    //     .nest("/apalis", apalis_api)
    //     .fallback_service(ServeUI::new())
    //     .layer(Extension(broadcaster.clone()));
    // app.add_router(router.into());
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
