use crate::utils::jwt::Claims;
use crate::views::statistics::{
    InstanceStatistics, StatisticsSummary, TaskStatistics, TimeSeriesData, UsageStatistics,
};
use anyhow::Context;
use sea_orm::{ConnectionTrait, DbConn, Statement};
use spring_web::axum::Json;
use spring_web::error::Result;
use spring_web::extractor::Component;
use spring_web::get_api;

/// # 获取统计信息
/// @tag statistics
#[get_api("/statistics")]
async fn get_statistics(
    claims: Claims,
    Component(db): Component<DbConn>,
) -> Result<Json<StatisticsSummary>> {
    let user_id = claims.uid;

    // 任务统计 - 使用SQL查询
    let total_sql = "SELECT COUNT(*)::bigint as count FROM scraper_task WHERE user_id = $1";
    let total: i64 = db
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            total_sql,
            [user_id.into()],
        ))
        .await
        .context("统计任务总数失败")?
        .and_then(|row| row.try_get("", "count").ok())
        .ok_or_else(|| anyhow::anyhow!("获取任务总数失败"))?;

    let undeployed_sql = "SELECT COUNT(*)::bigint as count FROM scraper_task WHERE user_id = $1 AND data IS NULL";
    let undeployed: i64 = db
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            undeployed_sql,
            [user_id.into()],
        ))
        .await
        .context("统计未部署任务数失败")?
        .and_then(|row| row.try_get("", "count").ok())
        .ok_or_else(|| anyhow::anyhow!("获取未部署任务数失败"))?;

    let scheduled_sql = "SELECT COUNT(*)::bigint as count FROM scraper_task WHERE user_id = $1 AND data IS NOT NULL AND deleted = false";
    let scheduled: i64 = db
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            scheduled_sql,
            [user_id.into()],
        ))
        .await
        .context("统计调度中任务数失败")?
        .and_then(|row| row.try_get("", "count").ok())
        .ok_or_else(|| anyhow::anyhow!("获取调度中任务数失败"))?;

    let completed_sql = "SELECT COUNT(*)::bigint as count FROM scraper_task WHERE user_id = $1 AND deleted = true";
    let completed: i64 = db
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            completed_sql,
            [user_id.into()],
        ))
        .await
        .context("统计调度结束任务数失败")?
        .and_then(|row| row.try_get("", "count").ok())
        .ok_or_else(|| anyhow::anyhow!("获取调度结束任务数失败"))?;

    // 实例统计（暂时基于任务数据，如果有独立的实例表可以后续优化）
    // 这里假设调度次数 = 有调度配置的任务数，失败次数需要从实例表中获取
    // 如果没有实例表，可以暂时返回0或基于其他逻辑计算
    let total_count = scheduled;
    
    // 查询失败次数（如果有实例表，应该从实例表中统计状态为FAILED的数量）
    // 这里暂时返回0，需要根据实际业务逻辑调整
    let failed_count = 0i64;

    // 使用统计
    // 数据表数量 = 任务数量
    let table_count = total;
    
    // 数据占用空间（MB），这里暂时返回0，需要根据实际存储逻辑计算
    let storage_size = 0.0;

    // 时间序列数据（最近30天的任务创建趋势）
    let time_series = get_time_series_data(&db, user_id)
        .await
        .context("获取时间序列数据失败")?;

    Ok(Json(StatisticsSummary {
        task_stats: TaskStatistics {
            total,
            undeployed,
            scheduled,
            completed,
        },
        instance_stats: InstanceStatistics {
            total_count,
            failed_count,
        },
        usage_stats: UsageStatistics {
            table_count,
            storage_size,
            time_series,
        },
    }))
}

/// 获取时间序列数据（最近30天）
async fn get_time_series_data(db: &DbConn, user_id: i64) -> anyhow::Result<Vec<TimeSeriesData>> {
    // 查询最近30天的任务创建统计
    let sql = r#"
        SELECT 
            DATE(created) as date,
            COUNT(*)::bigint as count
        FROM scraper_task
        WHERE user_id = $1
          AND created >= CURRENT_DATE - INTERVAL '30 days'
        GROUP BY DATE(created)
        ORDER BY date ASC
    "#;

    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        sql,
        [user_id.into()],
    );

    let result = db.query_all(stmt).await.context("查询时间序列数据失败")?;
    
    let mut time_series = Vec::new();
    for row in result {
        let date: sea_orm::sqlx::types::chrono::NaiveDate = row.try_get("", "date")?;
        let count: i64 = row.try_get("", "count")?;
        
        time_series.push(TimeSeriesData {
            date: date.format("%Y/%m/%d").to_string(),
            value: count as f64,
        });
    }

    // 如果数据不足30天，填充缺失的日期为0
    use sea_orm::sqlx::types::chrono::Local;
    let today = Local::now().date_naive();
    let mut filled_series = Vec::new();
    for i in 0..30 {
        let days_ago = 29 - i;
        // 使用SQL计算日期更简单，或者使用NaiveDate的pred_opt方法
        let date = if days_ago == 0 {
            today
        } else {
            // 通过减去天数来计算日期
            let mut date = today;
            for _ in 0..days_ago {
                date = date.pred_opt().unwrap_or(date);
            }
            date
        };
        let date_str = date.format("%Y/%m/%d").to_string();
        
        if let Some(existing) = time_series.iter().find(|d| d.date == date_str) {
            filled_series.push(TimeSeriesData {
                date: existing.date.clone(),
                value: existing.value,
            });
        } else {
            filled_series.push(TimeSeriesData {
                date: date_str,
                value: 0.0,
            });
        }
    }

    Ok(filled_series)
}

