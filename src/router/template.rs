use crate::{
    model::{
        favorite,
        prelude::{Favorite, TaskTemplate},
        task_template,
    },
    utils::jwt::{Claims, OptionalClaims},
    views::template::{ListTemplateResp, TemplateQuery},
};
use anyhow::Context;
use itertools::Itertools;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};
use spring_sea_orm::pagination::{Page, Pagination, PaginationExt};
use spring_web::{
    axum::Json,
    delete_api,
    error::{KnownWebError, Result},
    extractor::{Component, Path, Query},
    get_api, post_api,
};
use std::collections::HashSet;

/// # 查询任务模板
/// @tag template
#[get_api("/template")]
async fn query(
    claims: OptionalClaims,
    Component(db): Component<DbConn>,
    Query(query): Query<TemplateQuery>,
    pagination: Pagination,
) -> Result<Json<Page<ListTemplateResp>>> {
    let page = TaskTemplate::find()
        .filter(query)
        .page(&db, &pagination)
        .await
        .context("query task template page failed")?;
    if claims.is_none() {
        return Ok(Json(page.map(ListTemplateResp::from)));
    }
    let claims = claims.get()?;
    let tids = page.iter().map(|m| m.id).collect_vec();
    let favs = Favorite::find()
        .filter(
            favorite::Column::UserId
                .eq(claims.uid)
                .and(favorite::Column::TemplateId.is_in(tids)),
        )
        .all(&db)
        .await
        .context("query task template favorite failed")?
        .iter()
        .map(|m| m.template_id)
        .collect::<HashSet<i64>>();
    Ok(Json(page.map(|m| {
        let like = favs.contains(&m.id);
        ListTemplateResp::new(m, like)
    })))
}

/// # 我收藏的任务模板
/// @tag template
#[get_api("/template/favorite")]
async fn my_favorite(
    claims: Claims,
    Component(db): Component<DbConn>,
    pagination: Pagination,
) -> Result<Json<Page<ListTemplateResp>>> {
    let favs = Favorite::find()
        .filter(favorite::Column::UserId.eq(claims.uid))
        .page(&db, &pagination)
        .await
        .context("query my favorite failed")?;

    let tids = favs.iter().map(|m| m.template_id).collect_vec();
    let templates = TaskTemplate::find()
        .filter(task_template::Column::Id.is_in(tids))
        .all(&db)
        .await
        .context("query template by ids failed")?
        .into_iter()
        .map(|m| ListTemplateResp::new(m, true))
        .collect_vec();

    Ok(Json(Page::new(templates, &pagination, favs.total_elements)))
}

/// # 收藏任务模板
/// @tag template
#[post_api("/template/{template_id}/favorite")]
async fn add_favorite(
    Path(template_id): Path<i64>,
    claims: Claims,
    Component(db): Component<DbConn>,
) -> Result<Json<favorite::Model>> {
    let effect = TaskTemplate::incr_fav_count_by_id(&db, template_id)
        .await
        .context("increase fav_count failed")?;
    if effect <= 0 {
        Err(KnownWebError::not_found("模板不存在"))?;
    }
    let fav = favorite::ActiveModel {
        user_id: Set(claims.uid),
        template_id: Set(template_id),
        ..Default::default()
    }
    .insert(&db)
    .await
    .context("favorite save failed")?;
    Ok(Json(fav))
}

/// # 取消收藏任务模板
/// @tag template
#[delete_api("/template/{template_id}/favorite")]
async fn delete_favorite(
    Path(template_id): Path<i64>,
    claims: Claims,
    Component(db): Component<DbConn>,
) -> Result<Json<bool>> {
    let effect = TaskTemplate::desc_fav_count_by_id(&db, template_id)
        .await
        .context("decrease fav_count failed")?;
    if effect <= 0 {
        Err(KnownWebError::not_found("模板不存在"))?;
    }
    let result = favorite::ActiveModel {
        user_id: Set(claims.uid),
        template_id: Set(template_id),
        ..Default::default()
    }
    .delete(&db)
    .await
    .context("favorite delete failed")?;
    Ok(Json(result.rows_affected > 0))
}
