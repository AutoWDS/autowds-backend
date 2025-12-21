use crate::{
    model::{pay_order, sea_orm_active_enums::PayFrom},
    utils::pay_service::PayOrderService,
};
use chrono::{Duration, Local};
use sea_orm::DbConn;
use spring::{plugin::service::Service, tracing};
use spring_job::{cron, Job};

/// 支付订单状态检查任务
/// 每5分钟检查一次未确认的订单状态
#[derive(Clone, Service)]
pub struct PayOrderCheckJob {
    #[inject(component)]
    db: DbConn,
    #[inject(component)]
    pay_service: PayOrderService,
}

#[Job]
impl PayOrderCheckJob {
    #[cron("0 */5 * * * *")] // 每5分钟执行一次
    async fn check_pending_orders(&self) {
        tracing::info!("开始检查待确认的支付订单");

        // 查询30分钟前创建但未确认的订单
        let check_time = Local::now().naive_local() - Duration::minutes(30);
        
        match self.pay_service.find_wait_confirm_after(check_time).await {
            Ok(orders) => {
                tracing::info!("找到 {} 个待确认订单", orders.len());
                
                for order in orders {
                    match order.pay_from {
                        PayFrom::Alipay => {
                            if let Err(e) = self.pay_service.query_alipay_order(order.clone()).await {
                                tracing::error!("查询支付宝订单 {} 状态失败: {}", order.id, e);
                            } else {
                                tracing::info!("已更新支付宝订单 {} 状态", order.id);
                            }
                        }
                        PayFrom::Wechat => {
                            if let Err(e) = self.pay_service.query_wechat_order(order.clone()).await {
                                tracing::error!("查询微信订单 {} 状态失败: {}", order.id, e);
                            } else {
                                tracing::info!("已更新微信订单 {} 状态", order.id);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                tracing::error!("查询待确认订单失败: {}", e);
            }
        }
        
        tracing::info!("支付订单状态检查完成");
    }
}