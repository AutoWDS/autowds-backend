use crate::config::s3::TaskLogS3Config;
use crate::model::prelude::{AccountUser, ScraperTask, TaskInstance};
use crate::s3_task_log::fetch_archived_task_log_bytes;
use crate::model::task_instance;
use crate::model::scraper_task::{self, ScheduleData, ScraperTaskData};
use crate::model::sea_orm_active_enums::ProductEdition;
use crate::utils::jwt::{Claims, OptionalClaims};
use crate::views::task::{ScraperTaskQuery, ScraperTaskReq, ScraperUpdateTaskReq};
use crate::views::task_instance_capture::TaskInstanceCaptureItem;
use anyhow::Context;
use axum_valid::Valid;
use chrono::Local;
use futures_util::StreamExt;
use itertools::Itertools;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbConn, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use serde::Deserialize;
use serde_json::Value;
use std::convert::Infallible;
use std::sync::Arc;
use summer::config::ConfigRegistry;
use summer_job::job::Job;
use summer_job::JobScheduler;
use summer_redis::config::RedisConfig;
use summer_redis::redis;
use summer_sea_orm::pagination::{Page, Pagination, PaginationExt};
use summer_sqlx::sqlx;
use summer_sqlx::ConnectPool;
use summer_web::axum::response::sse::{Event, Sse};
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

    let mut body = body;
    if let Some(ref mut d) = body.data {
        if let Some(ref mut dq) = d.data_quality {
            scraper_task::apply_data_quality_dedupe_version(None, dq);
        }
    }

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
        .map(|mut m| {
            if let Some(ref mut d) = m.data {
                if let Some(ref mut dq) = d.data_quality {
                    scraper_task::apply_data_quality_dedupe_version(None, dq);
                }
            }
            scraper_task::ActiveModel {
                user_id: Set(claims.uid),
                name: Set(m.name),
                data: Set(m.data),
                rule: Set(m.rule),
                created: Set(now),
                modified: Set(now),
                deleted: Set(false),
                ..Default::default()
            }
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

    let mut body = body;
    if let Some(ref mut d) = body.data {
        if let Some(ref mut dq) = d.data_quality {
            scraper_task::apply_data_quality_dedupe_version(task.data.as_ref(), dq);
        }
    }

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

/// # 仅更新 cron 表达式（须已有 `data.schedule`；与 `PATCH /task/{id}/schedule` 相比不改 `proxyId` / `type`）
/// @tag task
///
/// 请求体为 JSON 字符串，例如 `"0 0 * * * *"`（与前端 `JSON.stringify(cron)` 一致）。
#[patch_api("/task/{id}/cron")]
async fn update_task_cron(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
    Component(sched): Component<JobScheduler>,
    AppRef(app): AppRef,
    Json(cron): Json<String>,
) -> Result<Json<i64>> {
    let task = ScraperTask::find_check_task(&db, id, claims.uid).await?;

    let cron_trim = cron.trim();
    if cron_trim.is_empty() {
        Err(KnownWebError::bad_request("cron 不能为空"))?;
    }

    let mut new_data = task
        .data
        .clone()
        .ok_or_else(|| {
            KnownWebError::bad_request(
                "任务尚未配置 data，请先使用 PATCH /task/{id}/schedule 配置调度",
            )
        })?;

    let schedule = new_data.schedule.as_mut().ok_or_else(|| {
        KnownWebError::bad_request(
            "任务尚未包含 schedule，请先使用 PATCH /task/{id}/schedule 提交完整调度配置",
        )
    })?;

    schedule.cron = cron_trim.to_string();

    let new_job_id = replace_cron_job(&sched, app, &task, cron_trim).await?;

    scraper_task::ActiveModel {
        id: Set(task.id),
        data: Set(Some(new_data)),
        job_id: Set(Some(new_job_id)),
        ..Default::default()
    }
    .save(&db)
    .await
    .context("save scraper task failed")?;

    Ok(Json(task.id))
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
    Ok(Json(
        task
            .data
            .as_ref()
            .and_then(|d| d.schedule.clone()),
    ))
}

async fn replace_cron_job(
    sched: &JobScheduler,
    app: Arc<summer::app::App>,
    task: &scraper_task::Model,
    cron: &str,
) -> anyhow::Result<summer_job::JobId> {
    if let Some(old_job_id) = task.job_id {
        if let Err(e) = sched.remove(&old_job_id).await {
            tracing::warn!("移除旧调度任务失败: {e:?}, job_id={old_job_id}");
        }
    }
    let job = Job::cron_with_data(cron, task.id)
        .run(crate::task::dispatch_task)
        .build(app);
    sched.add(job).await.context("添加调度失败")
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
    Json(schedule): Json<ScheduleData>,
) -> Result<Json<i64>> {
    let task = ScraperTask::find_check_task(&db, id, claims.uid).await?;

    let new_job_id = replace_cron_job(&sched, app, &task, &schedule.cron).await?;

    let new_data = match task.data {
        Some(mut env) => {
            env.schedule = Some(schedule);
            Some(env)
        }
        None => Some(ScraperTaskData {
            schedule: Some(schedule),
            data_quality: None,
        }),
    };

    scraper_task::ActiveModel {
        id: Set(task.id),
        data: Set(new_data),
        job_id: Set(Some(new_job_id)),
        ..Default::default()
    }
    .save(&db)
    .await
    .context("save scraper task failed")?;

    Ok(Json(task.id))
}

/// # 某次任务实例的执行日志（SSE）
///
/// - 若 `task_instance.log_key` 已写入且服务端 `[s3]` 配置完整：从对象存储拉取 NDJSON，按行推送后结束连接（不订阅 Redis）。
/// - 否则：订阅 Redis `task:{taskId}:{instanceId}:logs` 实时推送。
#[get("/task/{task_id}/instance/{instance_id}/logs")]
async fn task_instance_logs(
    opt_claims: OptionalClaims,
    Path((task_id, instance_id)): Path<(i64, i64)>,
    Component(db): Component<DbConn>,
    AppRef(app): AppRef,
) -> Result<Sse<ReceiverStream<std::result::Result<Event, Infallible>>>> {
    let claims = opt_claims.get()?;
    ScraperTask::find_check_task(&db, task_id, claims.uid).await?;

    let inst = TaskInstance::find_by_id(instance_id)
        .one(&db)
        .await
        .context("查询 task_instance 失败")?
        .ok_or_else(|| KnownWebError::not_found("实例不存在"))?;
    if inst.task_id != task_id {
        return Err(KnownWebError::bad_request("实例与任务不匹配"))?;
    }

    if let Some(log_key) = inst
        .log_key
        .as_deref()
        .map(str::trim)
        .filter(|k| !k.is_empty())
    {
        let s3_cfg = app.get_config::<TaskLogS3Config>().map_err(|e| {
            KnownWebError::internal_server_error(format!("读取 [s3] 配置失败：{e}"))
        })?;
        if !s3_cfg.is_configured() {
            return Err(KnownWebError::internal_server_error(
                "实例已写入归档键 log_key，但服务端 [s3] 未配置完整，无法从对象存储读取日志",
            ))?;
        }
        let bytes = fetch_archived_task_log_bytes(&s3_cfg, log_key)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, %log_key, "S3 GetObject 任务归档日志失败");
                KnownWebError::internal_server_error(format!("读取归档日志失败：{e}"))
            })?;
        let body = String::from_utf8_lossy(&bytes).into_owned();
        let (tx, rx) = tokio::sync::mpsc::channel::<std::result::Result<Event, Infallible>>(64);
        tokio::spawn(async move {
            for line in body.lines() {
                let t = line.trim_end();
                if t.is_empty() {
                    continue;
                }
                if tx.send(Ok(Event::default().data(t))).await.is_err() {
                    break;
                }
            }
        });
        return Ok(Sse::new(ReceiverStream::new(rx)));
    }

    let config = app
        .get_config::<RedisConfig>()
        .map_err(|e| KnownWebError::internal_server_error(format!("读取Redis配置失败: {e}")))?;

    let channel = format!("task:{task_id}:{instance_id}:logs");
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
        if let Err(e) = pubsub.subscribe(&channel).await {
            tracing::error!(%channel, "Redis订阅失败: {e}");
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
    #[serde(rename = "taskId")]
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

fn task_instance_record_shard_table(user_id: i64) -> Option<String> {
    if user_id <= 0 {
        return None;
    }
    let name = format!("task_instance_record_{user_id}");
    if name.chars().all(|c| {
        c.is_ascii_lowercase() || c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'
    }) {
        Some(name)
    } else {
        None
    }
}

fn is_pg_undefined_table(e: &summer_sqlx::sqlx::Error) -> bool {
    match e {
        summer_sqlx::sqlx::Error::Database(db) => db.code().is_some_and(|c| c == "42P01"),
        _ => false,
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct TaskInstanceCaptureQuery {
    #[serde(rename = "taskId")]
    task_id: i64,
    #[serde(rename = "instanceId")]
    instance_id: i64,
}

/// # 查询某次任务实例的采集记录（按用户分表 `task_instance_record_{userId}`，sqlx）
/// @tag task
#[get_api("/instance/data")]
async fn query_instance_capture_records(
    claims: Claims,
    Query(q): Query<TaskInstanceCaptureQuery>,
    Component(db): Component<DbConn>,
    Component(pool): Component<ConnectPool>,
    pagination: Pagination,
) -> Result<Json<Page<TaskInstanceCaptureItem>>> {
    let uid = claims.uid;
    let Some(table) = task_instance_record_shard_table(uid) else {
        return Ok(Json(pagination.empty_page()));
    };

    ScraperTask::find_check_task(&db, q.task_id, uid).await?;

    let inst = TaskInstance::find_by_id(q.instance_id)
        .one(&db)
        .await
        .context("查询 task_instance 失败")?
        .ok_or_else(|| KnownWebError::not_found("实例不存在"))?;
    if inst.task_id != q.task_id {
        return Err(KnownWebError::forbidden("无权访问该实例"))?;
    }

    let count_sql = format!(
        "SELECT COUNT(*)::bigint AS c FROM {table} WHERE task_id = $1 AND task_instance_id = $2"
    );

    let count: i64 = match sqlx::query_scalar(&count_sql)
        .bind(q.task_id)
        .bind(q.instance_id)
        .fetch_one(&pool)
        .await
    {
        Ok(n) => n,
        Err(e) => {
            if is_pg_undefined_table(&e) {
                return Ok(Json(Page::new(vec![], &pagination, 0)));
            }
            return Err(e).context("统计采集记录失败")?;
        }
    };

    let total = std::cmp::max(count, 0i64) as u64;
    let offset_i64 = i64::try_from(pagination.page.saturating_mul(pagination.size))
        .unwrap_or(i64::MAX);
    let limit_i64 = i64::try_from(pagination.size).unwrap_or(i64::MAX);

    let select_sql = format!(
        "SELECT id, task_id, task_instance_id, dedupe_rule_version, dedupe_key, payload, created_at \
         FROM {table} \
         WHERE task_id = $1 AND task_instance_id = $2 \
         ORDER BY id ASC \
         LIMIT $3 OFFSET $4"
    );

    let rows = match sqlx::query(&select_sql)
        .bind(q.task_id)
        .bind(q.instance_id)
        .bind(limit_i64)
        .bind(offset_i64)
        .fetch_all(&pool)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            if is_pg_undefined_table(&e) {
                return Ok(Json(Page::new(vec![], &pagination, 0)));
            }
            return Err(e).context("查询采集记录失败")?;
        }
    };

    let mut content = Vec::with_capacity(rows.len());
    for row in rows {
        content.push(
            TaskInstanceCaptureItem::try_from_row(&row)
                .context("解析采集记录行失败")?,
        );
    }

    Ok(Json(Page::new(content, &pagination, total)))
}
