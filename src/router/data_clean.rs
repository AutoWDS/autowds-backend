use crate::service::data_clean::{export_pipeline, preview_pipeline, validate_pipeline};
use crate::utils::jwt::Claims;
use crate::views::data_clean::{
    CleanExportReq, CleanExportResp, CleanPipelineReq, CleanPipelineSummary, CleanPreviewReq,
    CleanPreviewResp, CleanValidationResp, SaveCleanPipelineReq,
};
use anyhow::Context;
use axum_valid::Valid;
use chrono::{NaiveDateTime, Utc};
use serde_json::Value;
use summer_sqlx::sqlx::{self, Row};
use summer_sqlx::ConnectPool;
use summer_web::axum::Json;
use summer_web::error::{KnownWebError, Result};
use summer_web::extractor::{Component, Path};
use summer_web::{get_api, post_api};

/// # 查询数据清洗流程
/// @tag data-clean
#[get_api("/store/{store_id}/clean/pipelines")]
async fn query_clean_pipelines(
    claims: Claims,
    Path(store_id): Path<String>,
    Component(pool): Component<ConnectPool>,
) -> Result<Json<Vec<CleanPipelineSummary>>> {
    ensure_pipeline_table(&pool).await?;

    let rows = sqlx::query(
        "SELECT id, store_id, name, definition, created_at, modified_at \
         FROM data_clean_pipeline \
         WHERE user_id = $1 AND store_id = $2 \
         ORDER BY modified_at DESC",
    )
    .bind(claims.uid)
    .bind(&store_id)
    .fetch_all(&pool)
    .await
    .context("查询数据清洗流程失败")?;

    let mut pipelines = Vec::with_capacity(rows.len());
    for row in rows {
        pipelines.push(row_to_pipeline_summary(row).context("解析数据清洗流程失败")?);
    }
    Ok(Json(pipelines))
}

/// # 保存数据清洗流程
/// @tag data-clean
#[post_api("/store/{store_id}/clean/pipelines")]
async fn save_clean_pipeline(
    claims: Claims,
    Path(store_id): Path<String>,
    Component(pool): Component<ConnectPool>,
    Valid(Json(body)): Valid<Json<SaveCleanPipelineReq>>,
) -> Result<Json<CleanPipelineSummary>> {
    ensure_pipeline_table(&pool).await?;

    let validation = validate_pipeline(&body.pipeline);
    if !validation.valid {
        return Err(KnownWebError::bad_request("清洗流程校验失败"))?;
    }

    let now = Utc::now().naive_utc();
    let definition = serde_json::to_value(&body.pipeline).context("序列化清洗流程失败")?;
    let row = sqlx::query(
        "INSERT INTO data_clean_pipeline (user_id, store_id, name, definition, created_at, modified_at) \
         VALUES ($1, $2, $3, $4, $5, $6) \
         RETURNING id, store_id, name, definition, created_at, modified_at",
    )
    .bind(claims.uid)
    .bind(&store_id)
    .bind(&body.name)
    .bind(definition)
    .bind(now)
    .bind(now)
    .fetch_one(&pool)
    .await
    .context("保存数据清洗流程失败")?;

    Ok(Json(
        row_to_pipeline_summary(row).context("解析保存后的数据清洗流程失败")?,
    ))
}

/// # 校验数据清洗流程
/// @tag data-clean
#[post_api("/store/{store_id}/clean/validate")]
async fn validate_clean_pipeline(
    _claims: Claims,
    Path(_store_id): Path<String>,
    Json(body): Json<CleanPipelineReq>,
) -> Result<Json<CleanValidationResp>> {
    Ok(Json(validate_pipeline(&body.pipeline)))
}

/// # 预览数据清洗结果
/// @tag data-clean
#[post_api("/store/{store_id}/clean/preview")]
async fn preview_clean_pipeline(
    _claims: Claims,
    Path(_store_id): Path<String>,
    Json(body): Json<CleanPreviewReq>,
) -> Result<Json<CleanPreviewResp>> {
    if body.records.is_empty() {
        return Err(KnownWebError::bad_request(
            "预览需要提供样本数据，当前 Rust 后端尚未接入 store 数据读取",
        ))?;
    }
    let resp =
        preview_pipeline(&body.pipeline, body.records, body.limit).context("执行清洗预览失败")?;
    Ok(Json(resp))
}

/// # 导出数据清洗结果
/// @tag data-clean
#[post_api("/store/{store_id}/clean/export")]
async fn export_clean_pipeline_api(
    _claims: Claims,
    Path(store_id): Path<String>,
    Json(body): Json<CleanExportReq>,
) -> Result<Json<CleanExportResp>> {
    if body.records.is_empty() {
        return Err(KnownWebError::bad_request(
            "导出需要提供待清洗数据，当前 Rust 后端尚未接入 store 数据读取",
        ))?;
    }
    let resp = export_pipeline(&body.pipeline, body.records, body.format, &store_id)
        .context("执行清洗导出失败")?;
    Ok(Json(resp))
}

async fn ensure_pipeline_table(pool: &ConnectPool) -> Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS data_clean_pipeline (
            id bigserial PRIMARY KEY,
            user_id bigint NOT NULL,
            store_id varchar(64) NOT NULL,
            name varchar(80) NOT NULL,
            definition jsonb NOT NULL,
            created_at timestamp NOT NULL,
            modified_at timestamp NOT NULL
        )",
    )
    .execute(pool)
    .await
    .context("初始化数据清洗流程表失败")?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_data_clean_pipeline_user_store_modified \
         ON data_clean_pipeline(user_id, store_id, modified_at DESC)",
    )
    .execute(pool)
    .await
    .context("初始化数据清洗流程索引失败")?;
    Ok(())
}

fn row_to_pipeline_summary(
    row: summer_sqlx::sqlx::postgres::PgRow,
) -> anyhow::Result<CleanPipelineSummary> {
    let definition: Value = row.try_get("definition")?;
    let created_at: NaiveDateTime = row.try_get("created_at")?;
    let modified_at: NaiveDateTime = row.try_get("modified_at")?;
    Ok(CleanPipelineSummary {
        id: row.try_get("id")?,
        store_id: row.try_get("store_id")?,
        name: row.try_get("name")?,
        pipeline: serde_json::from_value(definition)?,
        created_at: created_at.and_utc().to_rfc3339(),
        modified_at: modified_at.and_utc().to_rfc3339(),
    })
}
