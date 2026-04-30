use crate::{
    model::{
        pay_order::{self, Entity as PayOrder},
        sea_orm_active_enums::OrderStatus,
    },
    router::pay_query::TradeCreateQuery,
    service::{pay::PayOrderService, user::UserService},
    utils::jwt::Claims,
};
use axum_extra::headers::HeaderMap;
use chrono::NaiveDate;
use sea_orm::DbConn;
use serde::Deserialize;
use serde_json::json;
use summer::tracing;
use summer_web::{
    axum::{
        self,
        http::StatusCode,
        response::{IntoResponse, Response},
        Form, Json,
    },
    extractor::{Component, Path, Query},
    get, post,
};

/// 创建支付订单（表单提交）
#[post("/pay/create")]
async fn create_trade(
    claims: Claims,
    Component(ps): Component<PayOrderService>,
    Form(trade): Form<TradeCreateQuery>,
) -> Result<Json<serde_json::Value>, Response> {
    let (order_id, qrcode_url) = ps
        .create_order(claims.uid, trade.level, trade.edition, trade.pay_from)
        .await
        .map_err(|e| {
            tracing::error!("创建订单失败: {e:?}");
            (StatusCode::INTERNAL_SERVER_ERROR, "创建订单失败").into_response()
        })?;

    let qrcode_url = qrcode_url
        .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, "二维码生成失败").into_response())?;

    Ok(Json(json!({
        "order_id": order_id,
        "qrcode_url": qrcode_url,
        "pay_from": trade.pay_from,
    })))
}

/// 查询订单支付状态
#[post("/pay/{order_id}/status")]
async fn pay_status(
    claims: Claims,
    Path(order_id): Path<i64>,
    Component(db): Component<DbConn>,
) -> Result<Json<OrderStatus>, Response> {
    let status = pay_order::Entity::find_order_status(&db, order_id, claims.uid)
        .await
        .map_err(|e| {
            tracing::error!("查询订单状态失败: {e:?}");
            (StatusCode::INTERNAL_SERVER_ERROR, "查询失败").into_response()
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "订单不存在").into_response())?;

    Ok(Json(status))
}

/// 微信支付回调
#[post("/pay/notify/wechat")]
async fn wechat_pay_callback(
    Component(ps): Component<PayOrderService>,
    Component(us): Component<UserService>,
    headers: HeaderMap,
    body: String,
) -> Result<Json<serde_json::Value>, Response> {
    let serial = headers
        .get("Wechatpay-Serial")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    let signature = headers
        .get("Wechatpay-Signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    let timestamp = headers
        .get("Wechatpay-Timestamp")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    let nonce = headers
        .get("Wechatpay-Nonce")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    let notify = match ps
        .wechat_verify_signature(serial, timestamp, nonce, signature, &body)
        .await
    {
        Err(e) => {
            tracing::error!(
                serial = serial,
                signature = signature,
                timestamp = timestamp,
                nonce = nonce,
                "微信支付回调验签失败: {e:#}"
            );
            return Ok(Json(json!({"code": "FAIL", "message": "验签失败"})));
        }
        Ok(notify) => notify,
    };

    let model = match ps.notify_wechat_pay(&notify).await {
        Err(e) => {
            tracing::error!("处理微信支付回调失败: {e:#}");
            return Ok(Json(json!({"code": "FAIL", "message": "处理失败"})));
        }
        Ok(model) => model,
    };

    let pay_order::Model { user_id, level, .. } = model;
    match us.confirm_user(user_id, level).await {
        Err(e) => {
            tracing::error!("confirm_user({user_id},{level:?}) failed>>>{e:?}");
        }
        Ok(u) => {
            tracing::info!("confirm_user({user_id},{level:?}) success>>>{u:?}");
        }
    }

    Ok(Json(json!({"code": "SUCCESS"})))
}

/// 支付宝支付回调
#[post("/pay/notify/alipay")]
async fn alipay_callback(
    Component(ps): Component<PayOrderService>,
    Component(us): Component<UserService>,
    body: axum::body::Bytes,
) -> Result<&'static str, Response> {
    if let Err(e) = ps.alipay_verify_sign(&body).await {
        tracing::error!("支付宝验签失败:{e:#}");
        return Ok("fail");
    }

    let model = match ps.notify_alipay(&body).await {
        Err(e) => {
            tracing::error!("处理支付宝回调失败: {e:#}");
            return Ok("fail");
        }
        Ok(model) => model,
    };

    let pay_order::Model { user_id, level, .. } = model;
    match us.confirm_user(user_id, level).await {
        Err(e) => {
            tracing::error!("confirm_user({user_id},{level:?}) failed>>>{e:?}");
        }
        Ok(u) => {
            tracing::info!("confirm_user({user_id},{level:?}) success>>>{u:?}");
        }
    }

    Ok("success")
}

/// Paddle 支付回调
#[post("/pay/notify/paddle")]
async fn paddle_callback(
    Component(ps): Component<PayOrderService>,
    Component(us): Component<UserService>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Result<Json<serde_json::Value>, Response> {
    let signature = headers
        .get("Paddle-Signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    let event = match ps.paddle_verify_signature(signature, &body) {
        Err(e) => {
            tracing::error!("Paddle 验签失败: {e:#}");
            return Err((StatusCode::UNAUTHORIZED, "验签失败").into_response());
        }
        Ok(event) => event,
    };

    let model = match ps.notify_paddle(&event).await {
        Err(e) => {
            tracing::error!("处理 Paddle 回调失败: {e:#}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "处理失败").into_response());
        }
        Ok(model) => model,
    };

    if model.status == OrderStatus::Paid {
        let pay_order::Model { user_id, level, .. } = model;
        match us.confirm_user(user_id, level).await {
            Err(e) => {
                tracing::error!("confirm_user({user_id},{level:?}) failed>>>{e:?}");
            }
            Ok(u) => {
                tracing::info!("confirm_user({user_id},{level:?}) success>>>{u:?}");
            }
        }
    }

    Ok(Json(json!({"ok": true})))
}

#[derive(Deserialize)]
pub struct PayStatsQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// 支付统计
#[get("/pay/stats")]
async fn pay_stats(
    Component(pay_service): Component<PayOrderService>,
    Query(query): Query<PayStatsQuery>,
) -> Result<Json<serde_json::Value>, Response> {
    let start_date = query
        .start_date
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());
    let end_date = query
        .end_date
        .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok());

    match PayOrder::stats_by_day(&pay_service.db, start_date, end_date).await {
        Ok(stats) => Ok(Json(serde_json::to_value(stats).unwrap())),
        Err(e) => {
            tracing::error!("获取支付统计失败: {e:?}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "获取统计失败").into_response())
        }
    }
}
