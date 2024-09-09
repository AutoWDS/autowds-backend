use crate::dto::task::ScraperTaskQuery;
use crate::model::prelude::ScraperTask;
use crate::model::scraper_task;
use crate::utils::jwt::Claims;
use anyhow::Context;
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter};
use spring_sea_orm::pagination::{Pagination, PaginationExt};
use spring_web::axum::Json;
use spring_web::error::Result;
use spring_web::extractor::{Component, Query};
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
async fn add_task() -> Result<impl IntoResponse> {
    Ok("")
}

#[post("/task/batch")]
async fn add_batch_task() -> Result<impl IntoResponse> {
    Ok("")
}

#[get("/task/:id")]
async fn get_task() -> Result<impl IntoResponse> {
    Ok("")
}

#[delete("/task/:id")]
async fn delete_task() -> Result<impl IntoResponse> {
    Ok("")
}

#[put("/task/:id")]
async fn update_task() -> Result<impl IntoResponse> {
    Ok("")
}

#[get("/task/:id/rule")]
async fn get_task_rule() -> Result<impl IntoResponse> {
    Ok("")
}

#[patch("/task/:id/rule")]
async fn update_task_rule() -> Result<impl IntoResponse> {
    Ok("")
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
