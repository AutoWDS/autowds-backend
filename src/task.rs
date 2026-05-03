use anyhow::Context as _;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};
use std::sync::Arc;
use summer::{
    app::{App, AppBuilder},
    extractor::Component,
    plugin::{ComponentRegistry, MutableComponentRegistry as _},
};
use summer_apalis::apalis_redis::{RedisConfig as ApalisRedisConfig, RedisStorage};
use summer_apalis::{
    apalis::prelude::Monitor,
    apalis_board::axum::{
        framework::{ApiBuilder, RegisterRoute as _},
        sse::{TracingBroadcaster, TracingSubscriber},
    },
};
use summer_apalis::{apalis::prelude::*, apalis_board::axum::ui::ServeUI};
use summer_job::extractor::Data;
use summer_job::job::Job;
use summer_job::JobScheduler;
use summer::config::ConfigRegistry as _;
use summer_redis::Redis;
use summer_web::{
    axum::{Extension, Router},
    WebConfigurator as _,
};
use crate::model::prelude::ScraperTask;
use crate::model::scraper_task;

mod pay_check;

use crate::config::apalis::ApalisConfig;

pub type TaskPublisher = RedisStorage<i64>;

pub fn add_storage(app: &mut AppBuilder, monitor: Monitor) -> Monitor {
    let broadcaster = TracingBroadcaster::create();
    let line_subscriber = TracingSubscriber::new(&broadcaster);
    app.add_layer(line_subscriber.layer());
    let redis = app.get_expect_component::<Redis>();
    let apalis_cfg = app
        .get_config::<ApalisConfig>()
        .expect("读取 [apalis] 配置失败（config/app.toml 中需有 [apalis] 段）");
    let storage = TaskPublisher::new_with_config(
        redis.clone(),
        ApalisRedisConfig::new(apalis_cfg.queue.as_str()),
    );
    app.add_component(storage.clone());
    let apalis_api = ApiBuilder::new(Router::new()).register(storage).build();
    let router = Router::new()
        .nest("/apalis", apalis_api)
        .nest_service("/apalis/ui", ServeUI::new())
        .layer(Extension(broadcaster.clone()));
    app.add_router(router.into());
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

/// 启动时从数据库恢复所有活跃的 cron 调度
/// 因为 SimpleJobCode 的闭包存储在内存中，服务重启后丢失
/// 需要重新注册闭包才能正常触发任务
pub fn recover_task_schedules(
    app: Arc<App>,
) -> Box<dyn std::future::Future<Output = summer::error::Result<String>> + Send> {
    Box::new(async move {
        let db = app.get_expect_component::<DbConn>();
        let sched = app.get_expect_component::<JobScheduler>();

        let tasks = ScraperTask::find()
            .filter(scraper_task::Column::JobId.is_not_null())
            .filter(scraper_task::Column::Deleted.eq(false))
            .all(&db)
            .await
            .context("query active tasks failed")?;

        tracing::info!("开始恢复任务调度，共 {} 个活跃任务", tasks.len());

        for task in tasks {
            let Some(data) = &task.data else {
                continue;
            };

            // 先移除旧的调度（闭包已丢失，元数据可能在 Postgres 中残留）
            if let Some(old_job_id) = task.job_id {
                if let Err(e) = sched.remove(&old_job_id).await {
                    tracing::warn!("移除旧调度任务失败: {e:?}, job_id={old_job_id}");
                }
            }

            // 重新注册调度
            let job = Job::cron_with_data(&data.cron, task.id)
                .run(dispatch_task)
                .build(app.clone());

            match sched.add(job).await {
                Ok(new_job_id) => {
                    // 更新数据库中的 job_id
                    scraper_task::ActiveModel {
                        id: Set(task.id),
                        job_id: Set(Some(new_job_id)),
                        ..Default::default()
                    }
                    .save(&db)
                    .await
                    .context("update task job_id failed")?;

                    tracing::info!(
                        "恢复任务调度: task_id={}, old_job_id={:?}, new_job_id={}",
                        task.id, task.job_id, new_job_id
                    );
                }
                Err(e) => {
                    tracing::error!("恢复任务调度失败: task_id={}, error={:?}", task.id, e);
                }
            }
        }

        Ok("task schedules recovered".to_string())
    })
}
