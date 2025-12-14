use crate::model::prelude::{AccountUser, ScraperTask};
use crate::model::scraper_task::{self, ScheduleData};
use crate::model::sea_orm_active_enums::ProductEdition;
use crate::utils::jwt::Claims;
use crate::views::task::{ScraperTaskQuery, ScraperTaskReq, ScraperUpdateTaskReq};
use anyhow::Context;
use itertools::Itertools;
use sea_orm::{
    sqlx::types::chrono::Local, ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, PaginatorTrait, QueryFilter, Set,
};
use serde_json::Value;
use spring_job::job::Job;
use spring_job::JobScheduler;
use spring_sea_orm::pagination::{Page, Pagination, PaginationExt};
use spring_web::axum::response::IntoResponse;
use spring_web::axum::Json;
use spring_web::error::{KnownWebError, Result};
use spring_web::extractor::{AppRef, Component, Path, Query};
use spring_web::{delete_api, get_api, patch_api, post_api, put_api};

/// 检查用户任务数量限制
async fn check_task_limit(db: &DbConn, user_id: i64, user_edition: Option<ProductEdition>) -> Result<()> {
    let current_count = ScraperTask::find()
        .filter(scraper_task::Column::UserId.eq(user_id))
        .filter(scraper_task::Column::Deleted.eq(false))
        .count(db)
        .await
        .context("count user tasks failed")?;

    let limit = match user_edition {
        None => 1, // 未登录用户限制1个任务
        Some(ProductEdition::L0) => 3, // L0用户限制3个任务
        Some(ProductEdition::L1) => 10, // L1用户限制10个任务
        Some(ProductEdition::L2) => 50, // L2用户限制50个任务
        Some(ProductEdition::L3) => 200, // L3用户限制200个任务
    };

    if current_count >= limit {
        let message = match user_edition {
            None => "未登录用户最多只能创建1个任务，请先登录",
            Some(ProductEdition::L0) => "免费用户最多只能创建3个任务，请升级到付费版本",
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
    let mut filter = scraper_task::Column::UserId.eq(claims.uid);
    filter = match q.name {
        Some(name) => filter.and(scraper_task::Column::Name.starts_with(name)),
        None => filter,
    };
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
    Json(body): Json<ScraperTaskReq>,
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
    Json(batch): Json<Vec<ScraperTaskReq>>,
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

    let limit = match user.edition {
        ProductEdition::L0 => 3,
        ProductEdition::L1 => 10,
        ProductEdition::L2 => 50,
        ProductEdition::L3 => 200,
    };

    if current_count + batch.len() as u64 > limit {
        let message = match user.edition {
            ProductEdition::L0 => "免费用户最多只能创建3个任务，请升级到付费版本",
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
    Ok(Json(r.last_insert_id))
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
) -> Result<Json<i64>> {
    let task = ScraperTask::find_check_task(&db, id, claims.uid).await?;

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
    Json(body): Json<ScraperUpdateTaskReq>,
) -> Result<Json<i64>> {
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

    Ok(Json(task.id))
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
    scraper_task::ActiveModel {
        id: Set(task.id),
        data: Set(Some(data)),
        ..Default::default()
    }
    .save(&db)
    .await
    .context("save scraper task failed")?;

    let job = Job::cron_with_data(&cron, task.id)
        .run(crate::task::dispatch_task)
        .build(app);
    sched.add(job).await.context("添加调度失败")?;

    Ok(Json(task.id))
}

