use crate::model::prelude::{AccountUser, ScraperTask, TaskInstance};
use crate::model::task_instance;
use crate::model::scraper_task::{self, ScheduleData};
use crate::model::sea_orm_active_enums::ProductEdition;
use crate::utils::jwt::{Claims, OptionalClaims};
use crate::views::task::{ScraperTaskQuery, ScraperTaskReq, ScraperUpdateTaskReq};
use anyhow::Context;
use axum_valid::Valid;
use chrono::Local;
use futures_util::StreamExt;
use itertools::Itertools;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbConn, EntityTrait, ExprTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use serde::Deserialize;
use serde_json::Value;
use std::convert::Infallible;
use summer::config::ConfigRegistry;
use summer_job::job::Job;
use summer_job::JobScheduler;
use summer_redis::config::RedisConfig;
use summer_redis::redis;
use summer_sea_orm::pagination::{Page, Pagination, PaginationExt};
use summer_web::axum::response::sse::{Event, Sse};
use summer_web::axum::response::IntoResponse;
use summer_web::axum::Json;
use summer_web::error::{KnownWebError, Result};
use summer_web::extractor::{AppRef, Component, Path, Query};
use summer_web::{delete_api, get, get_api, patch_api, post_api, put_api};
use tokio_stream::wrappers::ReceiverStream;

/// 检查用户任务数量限制
async fn check_task_limit(
    db: &DbConn,
    user_id: i64,
    user_edition: Option<ProductEdition>,
) -> Result<()> {
    let current_count = ScraperTask::find()
        .filter(scraper_task::Column::UserId.eq(user_id))
        .filter(scraper_task::Column::Deleted.eq(false))
        .count(db)
        .await
        .context("count user tasks failed")?;

    let limit = match user_edition {
        None => 1, // 未登录用户限制1个任务
        Some(ref edition) => edition.task_limit(),
    };

    if current_count >= limit {
        let message = match user_edition {
            None => "未登录用户最多只能创建1个任务，请先登录",
            Some(ProductEdition::L0) => "免费用户最多只能创建5个任务，请升级到付费版本",
            _ => "已达到当前版本的任务数量上限",
        };
        return Err(KnownWebError::forbidden(message))?;
    }

    Ok(())
}

/// # 查询当前用户的所有任务
/// @tag task
#[get_api("/task")]
async fn query_task(
    claims: Claims,
    Query(q): Query<ScraperTaskQuery>,
    Component(db): Component<DbConn>,
    pagination: Pagination,
) -> Result<Json<Page<scraper_task::Model>>> {
    let mut filter = Condition::all()
        .add(scraper_task::Column::UserId.eq(claims.uid))
        .add(scraper_task::Column::Deleted.eq(false));
    if let Some(name) = q.name {
        filter = filter.add(scraper_task::Column::Name.starts_with(name));
    }
    if let Some(start) = q.start_time {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&start) {
            filter = filter.add(scraper_task::Column::Created.gte(dt.naive_utc()));
        }
    }
    if let Some(end) = q.end_time {
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&end) {
            filter = filter.add(scraper_task::Column::Created.lte(dt.naive_utc()));
        }
    }
    let page = ScraperTask::find()
        .filter(filter)
        .page(&db, &pagination)
        .await
        .context("query scraper task failed")?;
    Ok(Json(page))
}

/// # 新增任务
/// @tag task
#[post_api("/task")]
async fn add_task(
    claims: Claims,
    Component(db): Component<DbConn>,
    Valid(Json(body)): Valid<Json<ScraperTaskReq>>,
) -> Result<Json<scraper_task::Model>> {
    // 获取用户信息以检查版本级别
    let user = AccountUser::find_by_id(claims.uid)
        .one(&db)
        .await
        .context("find user failed")?
        .ok_or_else(|| KnownWebError::not_found("用户不存在"))?;

    // 检查任务数量限制
    check_task_limit(&db, claims.uid, Some(user.edition)).await?;

    let task = scraper_task::ActiveModel {
        user_id: Set(claims.uid),
        name: Set(body.name),
        data: Set(body.data),
        rule: Set(body.rule),
        ..Default::default()
    }
    .insert(&db)
    .await
    .context("save scraper task failed")?;
    Ok(Json(task))
}

/// # 批量新增任务
/// @tag task
#[post_api("/task/batch")]
async fn add_batch_task(
    claims: Claims,
    Component(db): Component<DbConn>,
    Valid(Json(batch)): Valid<Json<Vec<ScraperTaskReq>>>,
) -> Result<Json<i64>> {
    if batch.len() > 10 {
        Err(KnownWebError::bad_request("任务过多无法保存"))?;
    }

    // 获取用户信息以检查版本级别
    let user = AccountUser::find_by_id(claims.uid)
        .one(&db)
        .await
        .context("find user failed")?
        .ok_or_else(|| KnownWebError::not_found("用户不存在"))?;

    // 检查当前任务数量
    let current_count = ScraperTask::find()
        .filter(scraper_task::Column::UserId.eq(claims.uid))
        .filter(scraper_task::Column::Deleted.eq(false))
        .count(&db)
        .await
        .context("count user tasks failed")?;

    let limit = user.edition.task_limit();

    if current_count + batch.len() as u64 > limit {
        let message = match user.edition {
            ProductEdition::L0 => "免费用户最多只能创建5个任务，请升级到付费版本",
            _ => "批量添加将超过当前版本的任务数量上限",
        };
        return Err(KnownWebError::forbidden(message))?;
    }

    let now = Local::now().naive_local();
    let batch = batch
        .into_iter()
        .map(|m| scraper_task::ActiveModel {
            user_id: Set(claims.uid),
            name: Set(m.name),
            data: Set(m.data),
            rule: Set(m.rule),
            created: Set(now),
            modified: Set(now),
            deleted: Set(false),
            ..Default::default()
        })
        .collect_vec();
    let r = ScraperTask::insert_many(batch)
        .exec(&db)
        .await
        .context("batch save scraper task failed")?;
    Ok(Json(r.last_insert_id.unwrap_or_default()))
}

/// # 获取任务详情
/// @tag task
#[get_api("/task/{id}")]
async fn get_task(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<Json<scraper_task::Model>> {
    let task = ScraperTask::find_check_task(&db, id, claims.uid).await?;

    Ok(Json(task))
}

/// # 删除任务
/// @tag task
#[delete_api("/task/{id}")]
async fn delete_task(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
    Component(sched): Component<JobScheduler>,
) -> Result<Json<i64>> {
    let task = ScraperTask::find_check_task(&db, id, claims.uid).await?;

    // 从调度器中移除关联的 cron 任务
    if let Some(job_id) = task.job_id {
        if let Err(e) = sched.remove(&job_id).await {
            tracing::warn!("移除调度任务失败: {e:?}, job_id={job_id}");
        }
    }

    scraper_task::ActiveModel {
        id: Set(task.id),
        deleted: Set(true),
        ..Default::default()
    }
    .save(&db)
    .await
    .context("save scraper task failed")?;

    Ok(Json(task.id))
}

/// # 更新任务
/// @tag task
#[put_api("/task/{id}")]
async fn update_task(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
    Valid(Json(body)): Valid<Json<ScraperUpdateTaskReq>>,
) -> Result<Json<scraper_task::Model>> {
    let task = ScraperTask::find_check_task(&db, id, claims.uid).await?;

    scraper_task::ActiveModel {
        id: Set(task.id),
        data: Set(body.data),
        rule: Set(body.rule),
        ..Default::default()
    }
    .save(&db)
    .await
    .context("save scraper task failed")?;

    Ok(Json(task))
}

/// # 获取任务规则
/// @tag task
#[get_api("/task/{id}/rule")]
async fn get_task_rule(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<Json<Value>> {
    let task = ScraperTask::find_check_task(&db, id, claims.uid).await?;

    Ok(Json(task.rule))
}

/// # 更新任务规则
/// @tag task
#[patch_api("/task/{id}/rule")]
async fn update_task_rule(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
    Json(rule): Json<Value>,
) -> Result<Json<i64>> {
    let task = ScraperTask::find_check_task(&db, id, claims.uid).await?;

    scraper_task::ActiveModel {
        id: Set(task.id),
        rule: Set(rule),
        ..Default::default()
    }
    .save(&db)
    .await
    .context("save scraper task failed")?;

    Ok(Json(task.id))
}

/// # 更新任务名
/// @tag task
#[patch_api("/task/{id}/name")]
async fn update_task_name(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
    Json(name): Json<String>,
) -> Result<Json<i64>> {
    let task = ScraperTask::find_check_task(&db, id, claims.uid).await?;

    scraper_task::ActiveModel {
        id: Set(task.id),
        name: Set(name),
        ..Default::default()
    }
    .save(&db)
    .await
    .context("save scraper task failed")?;

    Ok(Json(task.id))
}

// #[patch_api("/task/{id}/cron")]
async fn update_task_cron() -> Result<impl IntoResponse> {
    Ok("")
}

/// # 获取指定任务的调度配置
/// @tag task
#[get_api("/task/{id}/schedule")]
async fn get_task_schedule_info(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<Json<Option<ScheduleData>>> {
    let task = ScraperTask::find_check_task(&db, id, claims.uid).await?;
    Ok(Json(task.data))
}

/// # 修改指定任务的调度配置
/// @tag task
#[patch_api("/task/{id}/schedule")]
async fn update_task_schedule_info(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
    Component(sched): Component<JobScheduler>,
    AppRef(app): AppRef,
    Json(data): Json<ScheduleData>,
) -> Result<Json<i64>> {
    let task = ScraperTask::find_check_task(&db, id, claims.uid).await?;

    let cron = data.cron.clone();

    // 先移除旧的调度任务
    if let Some(old_job_id) = task.job_id {
        if let Err(e) = sched.remove(&old_job_id).await {
            tracing::warn!("移除旧调度任务失败: {e:?}, job_id={old_job_id}");
        }
    }

    // 添加新的调度任务
    let job = Job::cron_with_data(&cron, task.id)
        .run(crate::task::dispatch_task)
        .build(app);
    let new_job_id = sched.add(job).await.context("添加调度失败")?;

    scraper_task::ActiveModel {
        id: Set(task.id),
        data: Set(Some(data)),
        job_id: Set(Some(new_job_id)),
        ..Default::default()
    }
    .save(&db)
    .await
    .context("save scraper task failed")?;

    Ok(Json(task.id))
}

/// # 订阅任务执行日志（SSE）
#[get("/task/{id}/logs")]
async fn task_logs(
    opt_claims: OptionalClaims,
    Path(id): Path<i64>,
    AppRef(app): AppRef,
) -> Result<Sse<ReceiverStream<std::result::Result<Event, Infallible>>>> {
    let _claims = opt_claims.get()?;
    let config = app
        .get_config::<RedisConfig>()
        .map_err(|e| KnownWebError::internal_server_error(format!("读取Redis配置失败: {e}")))?;

    let (tx, rx) = tokio::sync::mpsc::channel::<std::result::Result<Event, Infallible>>(100);

    tokio::spawn(async move {
        let client = match redis::Client::open(config.uri.clone()) {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Redis client创建失败: {e}");
                return;
            }
        };
        let mut pubsub = match client.get_async_pubsub().await {
            Ok(ps) => ps,
            Err(e) => {
                tracing::error!("Redis pubsub连接失败: {e}");
                return;
            }
        };
        if let Err(e) = pubsub.subscribe(format!("task:{id}:logs")).await {
            tracing::error!("Redis订阅失败: {e}");
            return;
        }

        let mut msg_stream = pubsub.on_message();
        let mut check_interval = tokio::time::interval(tokio::time::Duration::from_secs(5));

        loop {
            tokio::select! {
                Some(msg) = msg_stream.next() => {
                    let payload: String = msg.get_payload::<String>().unwrap_or_default();
                    let event = Event::default().data(payload);
                    if tx.send(Ok(event)).await.is_err() {
                        break;
                    }
                }
                _ = check_interval.tick() => {
                    if tx.is_closed() {
                        break;
                    }
                }
            }
        }
    });

    Ok(Sse::new(ReceiverStream::new(rx)))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct TaskInstanceQuery {
    task_id: i64,
}

/// # 查询任务运行实例列表
/// @tag task
#[get_api("/instance")]
async fn query_task_instances(
    claims: Claims,
    Query(q): Query<TaskInstanceQuery>,
    Component(db): Component<DbConn>,
    pagination: Pagination,
) -> Result<Json<Page<task_instance::Model>>> {
    // 先校验任务归属
    ScraperTask::find_check_task(&db, q.task_id, claims.uid).await?;

    let page = TaskInstance::find()
        .filter(task_instance::Column::TaskId.eq(q.task_id))
        .order_by_desc(task_instance::Column::Created)
        .page(&db, &pagination)
        .await
        .context("query task instances failed")?;

    Ok(Json(page))
}
