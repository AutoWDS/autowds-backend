use crate::model::prelude::{ScraperTask, TaskInstance};
use crate::model::{scraper_task, task_instance};
use crate::utils::jwt::Claims;
use crate::views::store::{
    dataset_created_millis, DatasetDataItem, DatasetDataPage, DatasetDataQuery, DatasetField,
    DatasetMeta, DatasetQuery, DatasetType,
};
use anyhow::Context;
use sea_orm::{
    ColumnTrait, Condition, DbConn, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use serde_json::Value;
use std::collections::{BTreeSet, HashMap};
use summer_sea_orm::pagination::{Page, Pagination};
use summer_sqlx::sqlx::{self, Row};
use summer_sqlx::ConnectPool;
use summer_web::axum::Json;
use summer_web::error::{KnownWebError, Result};
use summer_web::extractor::{Component, Path, Query};
use summer_web::get_api;

/// # 查询任务级数据集列表
/// @tag store
#[get_api("/store")]
async fn query_dataset(
    claims: Claims,
    Query(q): Query<DatasetQuery>,
    Component(db): Component<DbConn>,
    Component(pool): Component<ConnectPool>,
    pagination: Pagination,
) -> Result<Json<Page<DatasetMeta>>> {
    let mut filter = Condition::all()
        .add(scraper_task::Column::UserId.eq(claims.uid))
        .add(scraper_task::Column::Deleted.eq(false));
    if let Some(name) = q.name.filter(|name| !name.trim().is_empty()) {
        filter = filter.add(scraper_task::Column::Name.contains(name.trim()));
    }

    let total = ScraperTask::find()
        .filter(filter.clone())
        .count(&db)
        .await
        .context("统计数据集失败")?;

    let tasks = ScraperTask::find()
        .filter(filter)
        .order_by_desc(scraper_task::Column::Created)
        .offset(pagination.page.saturating_mul(pagination.size))
        .limit(pagination.size)
        .all(&db)
        .await
        .context("查询数据集失败")?;

    let task_ids = tasks.iter().map(|task| task.id).collect::<Vec<_>>();
    let counts = dataset_record_counts(&db, &task_ids).await?;
    let bytes = dataset_payload_bytes_map(&pool, claims.uid, &task_ids)
        .await
        .unwrap_or_default();

    let mut content = Vec::with_capacity(tasks.len());
    for task in tasks {
        content.push(DatasetMeta {
            id: task.id.to_string(),
            user_id: claims.uid.to_string(),
            name: task.name,
            created: dataset_created_millis(task.created),
            count: counts.get(&task.id).copied().unwrap_or_default(),
            bytes: bytes.get(&task.id).copied().unwrap_or_default(),
            ty: DatasetType::DOC,
        });
    }

    Ok(Json(Page::new(content, &pagination, total)))
}

/// # 查询任务级数据集字段
/// @tag store
#[get_api("/store/{store_id}/schema")]
async fn query_dataset_schema(
    claims: Claims,
    Path(store_id): Path<String>,
    Component(db): Component<DbConn>,
    Component(pool): Component<ConnectPool>,
) -> Result<Json<Vec<DatasetField>>> {
    let task_id = parse_store_id(&store_id)?;
    ScraperTask::find_check_task(&db, task_id, claims.uid).await?;

    let Some(table) = task_instance_record_shard_table(claims.uid) else {
        return Ok(Json(vec![]));
    };
    let sql = format!("SELECT payload FROM {table} WHERE task_id = $1 ORDER BY id ASC LIMIT 100");
    let rows = match sqlx::query(&sql).bind(task_id).fetch_all(&pool).await {
        Ok(rows) => rows,
        Err(e) if is_pg_undefined_table(&e) => return Ok(Json(vec![])),
        Err(e) => return Err(e).context("查询数据集字段失败")?,
    };

    let mut fields = BTreeSet::new();
    for row in rows {
        let payload: Value = row.try_get("payload").context("解析数据集字段失败")?;
        if let Value::Object(map) = payload {
            fields.extend(map.keys().cloned());
        }
    }

    Ok(Json(
        fields
            .into_iter()
            .map(|name| DatasetField {
                name,
                default_value: String::new(),
            })
            .collect(),
    ))
}

/// # 查询任务级数据集记录
/// @tag store
#[get_api("/store/{store_id}/data")]
async fn query_dataset_data(
    claims: Claims,
    Path(store_id): Path<String>,
    Query(q): Query<DatasetDataQuery>,
    Component(db): Component<DbConn>,
    Component(pool): Component<ConnectPool>,
) -> Result<Json<DatasetDataPage>> {
    let task_id = parse_store_id(&store_id)?;
    ScraperTask::find_check_task(&db, task_id, claims.uid).await?;

    let size = q.size.unwrap_or(50).clamp(1, 200);
    let Some(table) = task_instance_record_shard_table(claims.uid) else {
        return Ok(Json(empty_data_page(size, q.desc)));
    };

    let count_sql = format!("SELECT COUNT(*)::bigint AS c FROM {table} WHERE task_id = $1");
    let total = match sqlx::query_scalar::<_, i64>(&count_sql)
        .bind(task_id)
        .fetch_one(&pool)
        .await
    {
        Ok(total) => total,
        Err(e) if is_pg_undefined_table(&e) => return Ok(Json(empty_data_page(size, q.desc))),
        Err(e) => return Err(e).context("统计数据集记录失败")?,
    };

    let offset = q.offset.unwrap_or_default();
    let select_sql = if q.desc {
        if offset > 0 {
            format!(
                "SELECT id, payload, created_at FROM {table} \
                 WHERE task_id = $1 AND id < $2 \
                 ORDER BY id DESC \
                 LIMIT $3"
            )
        } else {
            format!(
                "SELECT id, payload, created_at FROM {table} \
                 WHERE task_id = $1 \
                 ORDER BY id DESC \
                 LIMIT $2"
            )
        }
    } else if offset > 0 {
        format!(
            "SELECT id, payload, created_at FROM {table} \
             WHERE task_id = $1 AND id > $2 \
             ORDER BY id ASC \
             LIMIT $3"
        )
    } else {
        format!(
            "SELECT id, payload, created_at FROM {table} \
             WHERE task_id = $1 \
             ORDER BY id ASC \
             LIMIT $2"
        )
    };

    let rows = match if offset > 0 {
        sqlx::query(&select_sql)
            .bind(task_id)
            .bind(offset)
            .bind(size)
            .fetch_all(&pool)
            .await
    } else {
        sqlx::query(&select_sql)
            .bind(task_id)
            .bind(size)
            .fetch_all(&pool)
            .await
    } {
        Ok(rows) => rows,
        Err(e) if is_pg_undefined_table(&e) => return Ok(Json(empty_data_page(size, q.desc))),
        Err(e) => return Err(e).context("查询数据集记录失败")?,
    };

    let mut content = Vec::with_capacity(rows.len());
    for row in rows {
        content.push(DatasetDataItem::try_from_row(&row).context("解析数据集记录失败")?);
    }
    let next_offset = content
        .last()
        .and_then(|item| item.id.parse::<i64>().ok())
        .unwrap_or(offset);

    Ok(Json(DatasetDataPage {
        content,
        total,
        size,
        offset: next_offset,
        desc: q.desc,
    }))
}

fn parse_store_id(store_id: &str) -> Result<i64> {
    store_id
        .parse::<i64>()
        .map_err(|_| KnownWebError::bad_request("数据集 ID 不正确").into())
}

fn task_instance_record_shard_table(user_id: i64) -> Option<String> {
    if user_id <= 0 {
        return None;
    }
    let name = format!("task_instance_record_{user_id}");
    if name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
    {
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

#[derive(Debug, FromQueryResult)]
struct DatasetRecordCount {
    task_id: i64,
    count: i64,
}

async fn dataset_record_counts(db: &DbConn, task_ids: &[i64]) -> Result<HashMap<i64, i64>> {
    if task_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let rows = TaskInstance::find()
        .select_only()
        .column(task_instance::Column::TaskId)
        .column_as(task_instance::Column::DataCount.sum(), "count")
        .filter(task_instance::Column::TaskId.is_in(task_ids.iter().copied()))
        .group_by(task_instance::Column::TaskId)
        .into_model::<DatasetRecordCount>()
        .all(db)
        .await
        .context("统计数据集记录数失败")?;

    Ok(rows
        .into_iter()
        .map(|row| (row.task_id, row.count))
        .collect())
}

async fn dataset_payload_bytes_map(
    pool: &ConnectPool,
    user_id: i64,
    task_ids: &[i64],
) -> anyhow::Result<HashMap<i64, i64>> {
    if task_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let Some(table) = task_instance_record_shard_table(user_id) else {
        return Ok(HashMap::new());
    };
    let sql = format!(
        "SELECT task_id, COALESCE(SUM(octet_length(payload::text)), 0)::bigint AS bytes \
         FROM {table} \
         WHERE task_id = ANY($1) \
         GROUP BY task_id"
    );
    let rows = match sqlx::query(&sql).bind(task_ids).fetch_all(pool).await {
        Ok(rows) => rows,
        Err(e) if is_pg_undefined_table(&e) => return Ok(HashMap::new()),
        Err(e) => return Err(e).context("统计数据集大小失败"),
    };

    let mut bytes = HashMap::with_capacity(rows.len());
    for row in rows {
        bytes.insert(row.try_get("task_id")?, row.try_get("bytes")?);
    }
    Ok(bytes)
}

fn empty_data_page(size: i64, desc: bool) -> DatasetDataPage {
    DatasetDataPage {
        content: vec![],
        total: 0,
        size,
        offset: 0,
        desc,
    }
}
