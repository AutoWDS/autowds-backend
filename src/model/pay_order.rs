use sea_orm::{
    prelude::DateTime, sqlx::types::chrono::Local, ActiveModelBehavior, ActiveValue::Set,
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, FromQueryResult, QueryFilter, QuerySelect,
    Statement,
};
use serde::{Deserialize, Serialize};
use spring::{async_trait, plugin::ComponentRegistry, App};
use anyhow::Context;
use chrono::{Days, NaiveDate};

// 重新导出实体
pub use super::_entities::pay_order::*;
use super::_entities::sea_orm_active_enums::*;

#[derive(Debug, FromQueryResult, Serialize)]
pub struct PayStatsByDay {
    pub day: DateTime,
    pub paid_count: i64,
    pub paid_amount: i64,
    pub pending_count: i64,
    pub pending_amount: i64,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.created = Set(Local::now().naive_local());
            self.status = Set(OrderStatus::Created);
        }
        self.modified = Set(Local::now().naive_local());
        Ok(self)
    }

    async fn after_save<C>(model: Model, _db: &C, insert: bool) -> Result<Model, DbErr>
    where
        C: ConnectionTrait,
    {
        // 这里可以添加订单创建后的处理逻辑，比如发送通知等
        if insert {
            tracing::info!("新订单创建: {}", model.id);
        }
        Ok(model)
    }
}

impl Entity {
    pub async fn find_wait_confirm_after<C: ConnectionTrait>(
        db: &C,
        time: DateTime,
    ) -> anyhow::Result<Vec<Model>> {
        Entity::find()
            .filter(Column::Confirm.is_null().and(Column::Created.gt(time)))
            .all(db)
            .await
            .with_context(|| format!("find_wait_confirm({time:?}) failed"))
    }

    pub async fn find_order_status<C: ConnectionTrait>(
        db: &C,
        order_id: i32,
        user_id: i32,
    ) -> anyhow::Result<Option<OrderStatus>> {
        Entity::find()
            .select_only()
            .column(Column::Status)
            .filter(Column::Id.eq(order_id).and(Column::UserId.eq(user_id)))
            .into_tuple()
            .one(db)
            .await
            .with_context(|| format!("find_order_status({order_id},{user_id}) failed"))
    }

    pub async fn stats_by_day<C: ConnectionTrait>(
        db: &C,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
    ) -> anyhow::Result<Vec<PayStatsByDay>> {
        let db_backend = db.get_database_backend();

        // 默认最近30天
        let end = end_date.unwrap_or_else(|| Local::now().date_naive());
        let start = start_date.unwrap_or_else(|| {
            end.checked_sub_days(Days::new(30))
                .expect("date subtract overflow")
        });

        let monthly_amount = OrderLevel::Monthly.amount();
        let quarterly_amount = OrderLevel::Quarterly.amount();
        let half_year_amount = OrderLevel::HalfYear.amount();
        let annual_amount = OrderLevel::Annual.amount();

        let sql = format!(
            r#"
            WITH date_series AS (
                SELECT generate_series(
                    $1::date,
                    $2::date,
                    '1 day'::interval
                )::timestamp as day
            ),
            paid_stats AS (
                SELECT 
                    date_trunc('day', confirm) as day,
                    COUNT(*) as paid_count,
                    SUM(CASE 
                        WHEN level = 'monthly' THEN {monthly_amount}
                        WHEN level = 'quarterly' THEN {quarterly_amount}
                        WHEN level = 'half_year' THEN {half_year_amount}
                        WHEN level = 'annual' THEN {annual_amount}
                        ELSE 0
                    END) as paid_amount
                FROM pay_order
                WHERE status = 'paid' AND confirm IS NOT NULL
                  AND date_trunc('day', confirm) >= $1::date 
                  AND date_trunc('day', confirm) <= $2::date
                GROUP BY day
            ),
            pending_stats AS (
                SELECT 
                    date_trunc('day', created) as day,
                    COUNT(*) as pending_count,
                    SUM(CASE 
                        WHEN level = 'monthly' THEN {monthly_amount}
                        WHEN level = 'quarterly' THEN {quarterly_amount}
                        WHEN level = 'half_year' THEN {half_year_amount}
                        WHEN level = 'annual' THEN {annual_amount}
                        ELSE 0
                    END) as pending_amount
                FROM pay_order
                WHERE status = 'created'
                  AND date_trunc('day', created) >= $1::date 
                  AND date_trunc('day', created) <= $2::date
                GROUP BY day
            )
            SELECT 
                date_series.day,
                COALESCE(paid_stats.paid_count, 0) as paid_count,
                COALESCE(paid_stats.paid_amount, 0) as paid_amount,
                COALESCE(pending_stats.pending_count, 0) as pending_count,
                COALESCE(pending_stats.pending_amount, 0) as pending_amount
            FROM date_series
            LEFT JOIN paid_stats ON date_series.day = paid_stats.day
            LEFT JOIN pending_stats ON date_series.day = pending_stats.day
            ORDER BY date_series.day
            "#,
        );

        let stmt =
            Statement::from_sql_and_values(db_backend, sql, vec![start.into(), end.into()]);

        PayStatsByDay::find_by_statement(stmt)
            .all(db)
            .await
            .context("PayStatsByDay execute failed")
    }
}

// 订单级别枚举扩展
impl OrderLevel {
    pub fn title(&self) -> &'static str {
        match self {
            OrderLevel::Monthly => "月度",
            OrderLevel::Quarterly => "季度", 
            OrderLevel::HalfYear => "半年",
            OrderLevel::Annual => "年度",
        }
    }

    pub fn amount(&self) -> i32 {
        match self {
            OrderLevel::Monthly => 2900,    // 29元
            OrderLevel::Quarterly => 7900,  // 79元
            OrderLevel::HalfYear => 14900,  // 149元
            OrderLevel::Annual => 26900,    // 269元
        }
    }
}

// 订单状态枚举扩展
impl OrderStatus {
    pub fn from_alipay(status: &str) -> Self {
        match status {
            "TRADE_SUCCESS" | "TRADE_FINISHED" => OrderStatus::Paid,
            "TRADE_CLOSED" => OrderStatus::Closed,
            _ => OrderStatus::Created,
        }
    }

    pub fn from_wechat(status: &str) -> Self {
        match status {
            "SUCCESS" => OrderStatus::Paid,
            "CLOSED" => OrderStatus::Closed,
            _ => OrderStatus::Created,
        }
    }
}