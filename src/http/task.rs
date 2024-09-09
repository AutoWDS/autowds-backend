use crate::dto::task::{ScraperTaskQuery, ScraperTaskReq};
use crate::model::prelude::ScraperTask;
use crate::model::scraper_task;
use crate::utils::jwt::Claims;
use anyhow::Context;
use itertools::Itertools;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};
use serde_json::Value;
use spring_sea_orm::pagination::{Pagination, PaginationExt};
use spring_web::axum::Json;
use spring_web::error::{KnownWebError, Result};
use spring_web::extractor::{Component, Path, Query};
use spring_web::{axum::response::IntoResponse, get};
use spring_web::{delete, patch, post, put};

#[get("/task")]
async fn query_task(
    claims: Claims,
    Query(q): Query<ScraperTaskQuery>,
    Component(db): Component<DbConn>,
    pagination: Pagination,
) -> Result<impl IntoResponse> {
    let mut filter = scraper_task::Column::UserId.eq(claims.uid);
    filter = match q.name {
        Some(name) => filter.and(scraper_task::Column::Name.starts_with(name)),
        None => filter,
    };
    let page = ScraperTask::find()
        .filter(filter)
        .page(&db, pagination)
        .await
        .context("query scraper task failed")?;
    Ok(Json(page))
}

#[post("/task")]
async fn add_task(
    claims: Claims,
    Component(db): Component<DbConn>,
    Json(body): Json<ScraperTaskReq>,
) -> Result<impl IntoResponse> {
    let task = scraper_task::ActiveModel {
        user_id: Set(claims.uid),
        name: Set(body.name),
        rule: Set(body.rule),
        ..Default::default()
    }
    .insert(&db)
    .await
    .context("save scraper task failed")?;
    Ok(Json(task))
}

#[post("/task/batch")]
async fn add_batch_task(
    claims: Claims,
    Component(db): Component<DbConn>,
    Json(batch): Json<Vec<ScraperTaskReq>>,
) -> Result<impl IntoResponse> {
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

#[get("/task/:id")]
async fn get_task(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<impl IntoResponse> {
    let task = ScraperTask::find_check_task(&db, id, claims.uid).await?;

    Ok(Json(task))
}

#[delete("/task/:id")]
async fn delete_task(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<impl IntoResponse> {
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

#[get("/task/:id/rule")]
async fn get_task_rule(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<impl IntoResponse> {
    let task = ScraperTask::find_check_task(&db, id, claims.uid).await?;

    Ok(Json(task.rule))
}

#[patch("/task/:id/rule")]
async fn update_task_rule(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
    Json(rule): Json<Value>,
) -> Result<impl IntoResponse> {
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

#[patch("/task/:id/name")]
async fn update_task(
    claims: Claims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
    Json(name): Json<String>,
) -> Result<impl IntoResponse> {
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

#[patch("/task/:id/cron")]
async fn update_task_cron() -> Result<impl IntoResponse> {
    Ok("")
}

#[get("/task/:id/schedule")]
async fn get_task_schedule_info() -> Result<impl IntoResponse> {
    Ok("")
}

#[patch("/task/:id/schedule")]
async fn update_task_schedule_info() -> Result<impl IntoResponse> {
    Ok("")
}
