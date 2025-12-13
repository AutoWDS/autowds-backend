use crate::{
    model::{
        account_user, prelude::*, scraper_task, sea_orm_active_enums::ProductEdition,
        task_template,
    },
    utils::jwt::AdminClaims,
    views::admin::*,
};
use anyhow::Context;
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, DbConn, EntityTrait, QueryFilter, Set,
};
use spring_sea_orm::pagination::{Page, Pagination, PaginationExt};
use spring_web::{
    axum::Json,
    delete, get,
    error::{KnownWebError, Result},
    extractor::{Component, Path, Query},
    post, put,
};

// ==================== 用户管理接口 ====================

/// 获取用户列表
#[get("/admin/user/list")]
async fn get_user_list(
    _admin: AdminClaims,
    Query(query): Query<UserListQuery>,
    Component(db): Component<DbConn>,
    pagination: Pagination,
) -> Result<Json<Page<UserResp>>> {
    let mut filter = account_user::Column::Id.is_not_null();
    
    if let Some(keyword) = query.keyword {
        filter = filter.and(
            account_user::Column::Name
                .contains(&keyword)
                .or(account_user::Column::Email.contains(&keyword)),
        );
    }

    let page = AccountUser::find()
        .filter(filter)
        .page(&db, &pagination)
        .await
        .context("query user list failed")?;

    Ok(Json(page.map(UserResp::from)))
}

/// 创建用户
#[post("/admin/user/create")]
async fn create_user(
    _admin: AdminClaims,
    Component(db): Component<DbConn>,
    Json(req): Json<CreateUserReq>,
) -> Result<Json<UserResp>> {
    // 检查邮箱是否已存在
    let existing = AccountUser::find()
        .filter(account_user::Column::Email.eq(&req.email))
        .one(&db)
        .await
        .context("check email failed")?;

    if existing.is_some() {
        return Err(KnownWebError::bad_request("邮箱已被注册"))?;
    }

    let user = account_user::ActiveModel {
        id: NotSet,
        name: Set(req.username),
        email: Set(req.email),
        passwd: Set(req.password),
        locked: Set(false),
        edition: Set(ProductEdition::L0),
        last_login: Set(None),
        ..Default::default()
    }
    .insert(&db)
    .await
    .context("create user failed")?;

    Ok(Json(UserResp::from(user)))
}

/// 更新用户
#[put("/admin/user/{id}")]
async fn update_user(
    _admin: AdminClaims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
    Json(req): Json<UpdateUserReq>,
) -> Result<Json<UserResp>> {
    let user = AccountUser::find_by_id(id)
        .one(&db)
        .await
        .context("find user failed")?
        .ok_or_else(|| KnownWebError::not_found("用户不存在"))?;

    let user = account_user::ActiveModel {
        id: Set(user.id),
        name: Set(req.username),
        email: Set(req.email),
        locked: Set(req.locked.unwrap_or(user.locked)),
        edition: Set(req.edition.unwrap_or(user.edition)),
        ..Default::default()
    }
    .update(&db)
    .await
    .context("update user failed")?;

    Ok(Json(UserResp::from(user)))
}

/// 删除用户
#[delete("/admin/user/{id}")]
async fn delete_user(
    _admin: AdminClaims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<Json<bool>> {
    let user = AccountUser::find_by_id(id)
        .one(&db)
        .await
        .context("find user failed")?
        .ok_or_else(|| KnownWebError::not_found("用户不存在"))?;

    account_user::ActiveModel {
        id: Set(user.id),
        ..Default::default()
    }
    .delete(&db)
    .await
    .context("delete user failed")?;

    Ok(Json(true))
}

// ==================== 任务管理接口 ====================

/// 获取所有任务列表（管理员）
#[get("/admin/task/list")]
async fn get_task_list(
    _admin: AdminClaims,
    Query(query): Query<TaskListQuery>,
    Component(db): Component<DbConn>,
    pagination: Pagination,
) -> Result<Json<Page<TaskResp>>> {
    let mut filter = scraper_task::Column::Id.is_not_null();

    if let Some(status) = query.status {
        filter = match status.as_str() {
            "pending" => filter.and(scraper_task::Column::Data.is_null()),
            "running" => filter.and(
                scraper_task::Column::Data
                    .is_not_null()
                    .and(scraper_task::Column::Deleted.eq(false)),
            ),
            "completed" => filter.and(scraper_task::Column::Deleted.eq(true)),
            _ => filter,
        };
    }

    let page = ScraperTask::find()
        .filter(filter)
        .page(&db, &pagination)
        .await
        .context("query task list failed")?;

    Ok(Json(page.map(TaskResp::from)))
}

/// 创建任务（管理员）
#[post("/admin/task/create")]
async fn create_task(
    _admin: AdminClaims,
    Component(db): Component<DbConn>,
    Json(req): Json<CreateTaskReq>,
) -> Result<Json<TaskResp>> {
    let task = scraper_task::ActiveModel {
        user_id: Set(req.user_id),
        name: Set(req.name),
        data: Set(None),
        rule: Set(serde_json::json!({})),
        ..Default::default()
    }
    .insert(&db)
    .await
    .context("create task failed")?;

    Ok(Json(TaskResp::from(task)))
}

/// 更新任务（管理员）
#[put("/admin/task/{id}")]
async fn update_task(
    _admin: AdminClaims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
    Json(req): Json<UpdateTaskReq>,
) -> Result<Json<TaskResp>> {
    let task = ScraperTask::find_by_id(id)
        .one(&db)
        .await
        .context("find task failed")?
        .ok_or_else(|| KnownWebError::not_found("任务不存在"))?;

    let task = scraper_task::ActiveModel {
        id: Set(task.id),
        name: Set(req.name),
        ..Default::default()
    }
    .update(&db)
    .await
    .context("update task failed")?;

    Ok(Json(TaskResp::from(task)))
}

/// 删除任务（管理员）
#[delete("/admin/task/{id}")]
async fn delete_task(
    _admin: AdminClaims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<Json<bool>> {
    let task = ScraperTask::find_by_id(id)
        .one(&db)
        .await
        .context("find task failed")?
        .ok_or_else(|| KnownWebError::not_found("任务不存在"))?;

    scraper_task::ActiveModel {
        id: Set(task.id),
        ..Default::default()
    }
    .delete(&db)
    .await
    .context("delete task failed")?;

    Ok(Json(true))
}

/// 启动任务
#[post("/admin/task/{id}/start")]
async fn start_task(
    _admin: AdminClaims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<Json<bool>> {
    let task = ScraperTask::find_by_id(id)
        .one(&db)
        .await
        .context("find task failed")?
        .ok_or_else(|| KnownWebError::not_found("任务不存在"))?;

    scraper_task::ActiveModel {
        id: Set(task.id),
        deleted: Set(false),
        ..Default::default()
    }
    .update(&db)
    .await
    .context("start task failed")?;

    Ok(Json(true))
}

/// 停止任务
#[post("/admin/task/{id}/stop")]
async fn stop_task(
    _admin: AdminClaims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<Json<bool>> {
    let task = ScraperTask::find_by_id(id)
        .one(&db)
        .await
        .context("find task failed")?
        .ok_or_else(|| KnownWebError::not_found("任务不存在"))?;

    scraper_task::ActiveModel {
        id: Set(task.id),
        deleted: Set(true),
        ..Default::default()
    }
    .update(&db)
    .await
    .context("stop task failed")?;

    Ok(Json(true))
}

// ==================== 模板管理接口 ====================

/// 获取模板列表（管理员）
#[get("/admin/template/list")]
async fn get_template_list(
    _admin: AdminClaims,
    Component(db): Component<DbConn>,
    pagination: Pagination,
) -> Result<Json<Page<TemplateResp>>> {
    let page = TaskTemplate::find()
        .page(&db, &pagination)
        .await
        .context("query template list failed")?;

    Ok(Json(page.map(TemplateResp::from)))
}

/// 创建模板（管理员）
#[post("/admin/template/create")]
async fn create_template(
    _admin: AdminClaims,
    Component(db): Component<DbConn>,
    Json(req): Json<CreateTemplateReq>,
) -> Result<Json<TemplateResp>> {
    use crate::model::sea_orm_active_enums::TemplateTopic;
    
    let template = task_template::ActiveModel {
        name: Set(req.name),
        detail: Set(req.description.unwrap_or_default()),
        rule: Set(req.config.unwrap_or_else(|| serde_json::json!({}))),
        data: Set(serde_json::json!({})),
        fav_count: Set(0),
        topic: Set(TemplateTopic::Other),
        edition: Set(ProductEdition::L0),
        lang: Set("zh".to_string()),
        img: Set(String::new()),
        params: Set(None),
        ..Default::default()
    }
    .insert(&db)
    .await
    .context("create template failed")?;

    Ok(Json(TemplateResp::from(template)))
}

/// 更新模板（管理员）
#[put("/admin/template/{id}")]
async fn update_template(
    _admin: AdminClaims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
    Json(req): Json<UpdateTemplateReq>,
) -> Result<Json<TemplateResp>> {
    let template = TaskTemplate::find_by_id(id)
        .one(&db)
        .await
        .context("find template failed")?
        .ok_or_else(|| KnownWebError::not_found("模板不存在"))?;

    let template = task_template::ActiveModel {
        id: Set(template.id),
        name: Set(req.name),
        detail: Set(req.description.unwrap_or(template.detail)),
        rule: Set(req.config.unwrap_or(template.rule)),
        ..Default::default()
    }
    .update(&db)
    .await
    .context("update template failed")?;

    Ok(Json(TemplateResp::from(template)))
}

/// 删除模板（管理员）
#[delete("/admin/template/{id}")]
async fn delete_template(
    _admin: AdminClaims,
    Path(id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<Json<bool>> {
    let template = TaskTemplate::find_by_id(id)
        .one(&db)
        .await
        .context("find template failed")?
        .ok_or_else(|| KnownWebError::not_found("模板不存在"))?;

    task_template::ActiveModel {
        id: Set(template.id),
        ..Default::default()
    }
    .delete(&db)
    .await
    .context("delete template failed")?;

    Ok(Json(true))
}

// ==================== 统计接口 ====================

/// 获取任务统计（管理员）
#[get("/admin/statistics/tasks")]
async fn get_task_statistics(
    _admin: AdminClaims,
    Component(db): Component<DbConn>,
) -> Result<Json<TaskStatisticsResp>> {
    use sea_orm::{ConnectionTrait, Statement};

    // 统计待执行任务
    let pending_sql = "SELECT COUNT(*)::bigint as count FROM scraper_task WHERE data IS NULL";
    let pending: i64 = db
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            pending_sql,
            [],
        ))
        .await
        .context("统计待执行任务失败")?
        .and_then(|row| row.try_get("", "count").ok())
        .unwrap_or(0);

    // 统计运行中任务
    let running_sql =
        "SELECT COUNT(*)::bigint as count FROM scraper_task WHERE data IS NOT NULL AND deleted = false";
    let running: i64 = db
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            running_sql,
            [],
        ))
        .await
        .context("统计运行中任务失败")?
        .and_then(|row| row.try_get("", "count").ok())
        .unwrap_or(0);

    // 统计已完成任务
    let completed_sql = "SELECT COUNT(*)::bigint as count FROM scraper_task WHERE deleted = true";
    let completed: i64 = db
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            completed_sql,
            [],
        ))
        .await
        .context("统计已完成任务失败")?
        .and_then(|row| row.try_get("", "count").ok())
        .unwrap_or(0);

    // 失败任务暂时返回0（需要根据实际业务逻辑调整）
    let failed = 0i64;

    Ok(Json(TaskStatisticsResp {
        pending,
        running,
        completed,
        failed,
    }))
}

/// 获取统计概览（管理员）
#[get("/admin/statistics/overview")]
async fn get_statistics_overview(
    _admin: AdminClaims,
    Component(db): Component<DbConn>,
) -> Result<Json<StatisticsOverviewResp>> {
    use sea_orm::{ConnectionTrait, Statement};

    // 统计用户总数
    let user_count_sql = "SELECT COUNT(*)::bigint as count FROM account_user";
    let user_count: i64 = db
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            user_count_sql,
            [],
        ))
        .await
        .context("统计用户总数失败")?
        .and_then(|row| row.try_get("", "count").ok())
        .unwrap_or(0);

    // 统计任务总数
    let task_count_sql = "SELECT COUNT(*)::bigint as count FROM scraper_task";
    let task_count: i64 = db
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            task_count_sql,
            [],
        ))
        .await
        .context("统计任务总数失败")?
        .and_then(|row| row.try_get("", "count").ok())
        .unwrap_or(0);

    // 统计模板总数
    let template_count_sql = "SELECT COUNT(*)::bigint as count FROM task_template";
    let template_count: i64 = db
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            template_count_sql,
            [],
        ))
        .await
        .context("统计模板总数失败")?
        .and_then(|row| row.try_get("", "count").ok())
        .unwrap_or(0);

    Ok(Json(StatisticsOverviewResp {
        user_count,
        task_count,
        template_count,
    }))
}
