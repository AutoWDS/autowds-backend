use crate::model::prelude::ScraperTask;
use crate::model::scraper_task;
use crate::utils::jwt::Claims;
use crate::views::task::{ScraperTaskQuery, ScraperTaskReq, ScraperUpdateTaskReq};
use anyhow::Context;
use itertools::Itertools;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};
use serde_json::Value;
use spring_sea_orm::pagination::{Page, Pagination, PaginationExt};
use spring_web::axum::response::IntoResponse;
use spring_web::axum::Json;
use spring_web::error::{KnownWebError, Result};
use spring_web::extractor::{Component, Path, Query};
use spring_web::{delete_api, get_api, patch_api, post_api, put_api};

/// 查询当前用户的所有任务
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

/// 新增任务
/// @tag task
#[post_api("/task")]
async fn add_task(
    claims: Claims,
    Component(db): Component<DbConn>,
    Json(body): Json<ScraperTaskReq>,
) -> Result<Json<scraper_task::Model>> {
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

/// 批量新增任务
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
    let batch = batch
        .into_iter()
        .map(|m| scraper_task::ActiveModel {
            user_id: Set(claims.uid),
            name: Set(m.name),
            rule: Set(m.rule),
            ..Default::default()
        })
        .collect_vec();
    let r = ScraperTask::insert_many(batch)
        .exec(&db)
        .await
        .context("batch save scraper task failed")?;
    Ok(Json(r.last_insert_id))
}

/// 获取任务详情
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

/// 删除任务
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

/// 更新任务
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

/// 获取任务规则
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

/// 更新任务规则
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

/// 更新任务名
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

// #[get_api("/task/{id}/schedule")]
async fn get_task_schedule_info() -> Result<impl IntoResponse> {
    Ok("")
}

// #[patch_api("/task/{id}/schedule")]
async fn update_task_schedule_info() -> Result<impl IntoResponse> {
    Ok("")
}
