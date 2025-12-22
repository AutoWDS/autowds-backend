use crate::{
    config::pay::PayConfig,
    model::{pay_order, sea_orm_active_enums::{OrderLevel, OrderStatus, PayFrom}},
    utils::pay_plugin::{Alipay, WechatPayClient},
};
use alipay_sdk_rust::{biz, response::TradePrecreateResponse};
use anyhow::{anyhow, Context};
use sea_orm::{
    prelude::DateTime, sqlx::types::chrono::Local, ActiveModelTrait, ActiveValue::Set, DbConn,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use spring::{plugin::service::Service, tracing};
use std::{collections::HashMap, env, fs::File, io::Write as _, path::Path};
use wechat_pay_rust_sdk::{
    model::{NativeParams, WechatPayDecodeData, WechatPayNotify},
    pay::PayNotifyTrait,
    response::{NativeResponse, ResponseTrait},
};

#[derive(Clone, Service)]
pub struct PayOrderService {
    #[inject(component)]
    pub db: DbConn,
    #[inject(component)]
    alipay: Alipay,
    #[inject(component)]
    wechat: WechatPayClient,
    #[inject(config)]
    config: PayConfig,
}

impl PayOrderService {
    pub async fn create_order(
        &self,
        uid: i64,
        level: OrderLevel,
        from: PayFrom,
    ) -> anyhow::Result<(i64, Option<String>)> {
        let order = pay_order::ActiveModel {
            user_id: Set(uid),
            level: Set(level),
            pay_from: Set(from),
            ..Default::default()
        }
        .insert(&self.db)
        .await
        .context("创建订单失败")?;

        let subject = format!("AutoWDS{}会员", level.title());
        let order_id = order.id;
        let amount = if self.config.test_pay_amount {
            1 // 1分钱
        } else {
            level.amount()
        };
        let qrcode_url = match from {
            PayFrom::Alipay => self.alipay(subject, order_id, amount).await?,
            PayFrom::Wechat => self.wechat_pay(subject, order_id, amount).await?,
        };
        Ok((order_id, qrcode_url))
    }

    async fn wechat_pay(
        &self,
        subject: String,
        order_id: i64,
        amount: i32,
    ) -> anyhow::Result<Option<String>> {
        let out_trade_no = format!("{:06}", order_id); // 微信"商户订单号"字符串规则校验最少6字节
        let wechat = self.wechat.clone();
        let resp = wechat
            .native_pay(NativeParams::new(subject, out_trade_no, amount.into()))
            .await
            .context("微信订单创建失败")?;
        let NativeResponse {
            code_url,
            code,
            message,
        } = resp;
        tracing::info!("wechat pay resp code ==> {code:?}, message ==> {message:?}");
        Ok(code_url)
    }

    async fn alipay(
        &self,
        subject: String,
        out_trade_no: i64,
        amount: i32,
    ) -> anyhow::Result<Option<String>> {
        let alipay = self.alipay.clone();
        let mut biz_content = biz::TradePrecreateBiz::new();
        biz_content.set_subject(subject.into());
        biz_content.set_out_trade_no(out_trade_no.into());
        biz_content.set_total_amount((amount as f64 / 100.0).into());
        let resp = alipay
            .trade_precreate(&biz_content)
            .context("支付宝订单创建失败")?;
        let resp_json = serde_json::to_value(&resp).context("支付宝响应出错")?;
        pay_order::ActiveModel {
            id: Set(out_trade_no),
            resp: Set(Some(resp_json)),
            ..Default::default()
        }
        .update(&self.db)
        .await
        .context("更新订单响应失败")?;
        let TradePrecreateResponse {
            response,
            alipay_cert_sn,
            sign,
        } = resp;
        tracing::info!("alipay resp sign ==> {sign:?}, alipay_cert_sn ==> {alipay_cert_sn:?}");
        Ok(response.qr_code)
    }

    pub async fn query_alipay_order(&self, model: pay_order::Model) -> anyhow::Result<pay_order::Model> {
        let order_id = model.id;
        let alipay = self.alipay.clone();
        let mut biz_content = biz::TradeQueryBiz::new();
        biz_content.set_out_trade_no(order_id.into());

        let resp = alipay
            .trade_query(&biz_content)
            .context("支付宝订单查询失败")?;

        let status_str = resp.response.trade_status.clone().unwrap_or_default();

        tracing::info!("支付宝订单#{order_id}状态: {status_str}");

        let status = OrderStatus::from_alipay(&status_str);
        let now = Local::now().naive_local();

        let model = pay_order::ActiveModel {
            id: Set(order_id),
            confirm: Set(Some(now)),
            status: Set(status),
            resp: Set(Some(
                serde_json::to_value(resp).context("resp to json failed")?,
            )),
            ..Default::default()
        }
        .update(&self.db)
        .await
        .with_context(|| format!("update_pay_order({order_id}) failed"))?;

        Ok(model)
    }

    pub async fn query_wechat_order(&self, model: pay_order::Model) -> anyhow::Result<pay_order::Model> {
        let wechat = self.wechat.clone();
        let order_id = model.id;
        let mchid = &wechat.mch_id;
        let resp = wechat
            .get_pay::<WechatPayOrderResp>(&format!(
                "/v3/pay/transactions/out-trade-no/{order_id:06}?mchid={mchid}"
            ))
            .await
            .context("微信订单查询失败")?;

        tracing::info!(
            "微信订单#{order_id}状态: {}({})",
            resp.trade_state,
            resp.trade_state_desc
        );

        let status = OrderStatus::from_wechat(&resp.trade_state);
        let now = Local::now().naive_local();

        let model = pay_order::ActiveModel {
            id: Set(order_id),
            confirm: Set(Some(now)),
            status: Set(status),
            resp: Set(Some(
                serde_json::to_value(resp).context("resp to json failed")?,
            )),
            ..Default::default()
        }
        .update(&self.db)
        .await
        .with_context(|| format!("update_pay_order({order_id}) failed"))?;

        Ok(model)
    }

    pub async fn alipay_verify_sign(&self, raw_body: &[u8]) -> anyhow::Result<()> {
        let alipay = self.alipay.clone();
        let r = alipay
            .async_verify_sign(raw_body)
            .context("支付宝验签失败")?;
        if r {
            Ok(())
        } else {
            Err(anyhow!("支付宝验签失败"))
        }
    }

    pub async fn wechat_verify_signature(
        &self,
        serial: &str,
        timestamp: &str,
        nonce: &str,
        signature: &str,
        body: &str,
    ) -> anyhow::Result<WechatPayNotify> {
        let wechat = self.wechat.clone();
        let pub_key = self.get_wechat_pub_key(serial).await?;

        wechat
            .verify_signature(&pub_key, timestamp, nonce, signature, body)
            .context("微信验签失败，非法数据")?;

        serde_json::from_str::<WechatPayNotify>(body).context("微信回调数据解析失败")
    }

    pub async fn get_wechat_pub_key(&self, serial: &str) -> anyhow::Result<String> {
        let pub_key_dir =
            env::var("WECHAT_PAY_PUB_KEY_DIR").unwrap_or("/data/wechat-cert/pubkey".to_string());
        let cert_dir = Path::new(&pub_key_dir);
        if !cert_dir.exists() {
            std::fs::create_dir_all(cert_dir)
                .with_context(|| format!("create dir {cert_dir:?} failed"))?;
        }
        let cert_path = format!("{pub_key_dir}/{serial}/pubkey.pem");
        let cert_path = Path::new(&cert_path);
        if cert_path.exists() {
            let pub_key = std::fs::read_to_string(cert_path)
                .with_context(|| format!("read pub key from {cert_path:?} failed"))?;
            return Ok(pub_key);
        }

        let wechat = self.wechat.clone();
        tracing::info!("fetch wechat pay certificates from wechat server");

        let resp = wechat
            .certificates()
            .await
            .context("获取微信平台证书失败")?;

        let certs = resp.data.ok_or_else(|| anyhow!("微信平台证书为空"))?;

        for cert in certs {
            let serial_no = cert.serial_no;
            let ciphertext = cert.encrypt_certificate.ciphertext;
            let nonce = cert.encrypt_certificate.nonce;
            let associated_data = cert.encrypt_certificate.associated_data;
            let data = wechat
                .decrypt_bytes(ciphertext, nonce, associated_data)
                .context("微信平台证书解密失败")?;
            let pub_key = wechat_pay_rust_sdk::util::x509_to_pem(data.as_slice())
                .map_err(|e| anyhow!("微信平台证书转换PEM失败:{e}"))?;
            let cert_path = format!("{pub_key_dir}/{serial_no}/pubkey.pem");
            let mut pub_key_file = File::create(cert_path).context("create pub key file failed")?;
            pub_key_file
                .write_all(pub_key.as_bytes())
                .context("write pub key file failed")?;

            let (pub_key_valid, expire_timestamp) =
                wechat_pay_rust_sdk::util::x509_is_valid(data.as_slice())
                    .map_err(|e| anyhow!("公钥验证失败:{e}"))?;
            tracing::debug!(
                "pub key valid:{} expire_timestamp:{}",
                pub_key_valid,
                expire_timestamp
            ); //检测证书是否可用,打印过期时间
        }

        let cert_path = format!("{pub_key_dir}/{serial}/pubkey.pem");
        let cert_path = Path::new(&cert_path);
        if cert_path.exists() {
            let pub_key = std::fs::read_to_string(cert_path)
                .with_context(|| format!("read pub key from {cert_path:?} failed"))?;
            return Ok(pub_key);
        } else {
            return Err(anyhow!("微信公钥不存在"));
        }
    }

    pub async fn notify_wechat_pay(
        &self,
        notify: &WechatPayNotify,
    ) -> anyhow::Result<pay_order::Model> {
        let wechat = self.wechat.clone();
        let resource = notify.resource.clone();
        let nonce = resource.nonce;
        let ciphertext = resource.ciphertext;
        let associated_data = resource.associated_data.unwrap_or_default();
        let data: WechatPayDecodeData = wechat
            .decrypt_paydata(
                ciphertext,      //加密数据
                nonce,           //随机串
                associated_data, //关联数据
            )
            .context("解析关联数据失败")?;

        tracing::info!("接收到微信订单状态: {}", data.trade_state);

        let status = OrderStatus::from_wechat(&data.trade_state);
        let out_trade_no = data.out_trade_no.parse::<i64>().context("解析订单号失败")?;
        let now = Local::now().naive_local();

        let model = pay_order::ActiveModel {
            id: Set(out_trade_no),
            confirm: Set(Some(now)),
            status: Set(status),
            resp: Set(Some(
                serde_json::to_value(notify).context("resp to json failed")?,
            )),
            ..Default::default()
        }
        .update(&self.db)
        .await
        .with_context(|| format!("update_pay_order({out_trade_no}) failed"))?;
        Ok(model)
    }

    pub async fn notify_alipay(&self, raw_body: &[u8]) -> anyhow::Result<pay_order::Model> {
        let notify = serde_urlencoded::from_bytes::<AlipayNotify>(raw_body)
            .context("支付宝notify解析失败")?;

        tracing::info!("接收到支付宝订单状态: {}", notify.trade_status);

        let out_trade_no = notify.out_trade_no;
        let status = OrderStatus::from_alipay(&notify.trade_status);
        let now = Local::now().naive_local();

        let model = pay_order::ActiveModel {
            id: Set(out_trade_no),
            confirm: Set(Some(now)),
            status: Set(status),
            resp: Set(Some(
                serde_json::to_value(notify).context("resp to json failed")?,
            )),
            ..Default::default()
        }
        .update(&self.db)
        .await
        .with_context(|| format!("update_pay_order({out_trade_no}) failed"))?;

        Ok(model)
    }

    pub async fn find_wait_confirm_after(
        &self,
        after_time: DateTime,
    ) -> anyhow::Result<Vec<pay_order::Model>> {
        pay_order::Entity::find_wait_confirm_after(&self.db, after_time).await
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WechatPayOrderResp {
    pub appid: String,
    pub mchid: String,
    pub out_trade_no: String,
    pub trade_state: String,
    pub trade_state_desc: String,
    pub transaction_id: Option<String>,
    pub trade_type: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl ResponseTrait for WechatPayOrderResp {}

/// 支付宝异步通知参数
#[derive(Debug, Serialize, Deserialize)]
pub struct AlipayNotify {
    pub notify_time: String,
    pub notify_type: String,
    pub notify_id: String,
    pub sign_type: String,
    pub sign: String,

    pub trade_no: String,
    pub app_id: String,
    pub auth_app_id: String,
    pub out_trade_no: i64,
    pub out_biz_no: Option<String>,

    #[serde(alias = "buyer_id", alias = "buyer_open_id")]
    pub buyer_id: Option<String>,
    pub buyer_logon_id: Option<String>,
    pub seller_id: Option<String>,
    pub seller_email: Option<String>,

    pub trade_status: String,
    pub total_amount: String,
    pub receipt_amount: Option<String>,
    pub invoice_amount: Option<String>,
    pub buyer_pay_amount: Option<String>,
    pub point_amount: Option<String>,
    pub refund_fee: Option<String>,
    pub send_back_fee: Option<String>,

    pub subject: Option<String>,
    pub body: Option<String>,

    pub gmt_create: Option<String>,
    pub gmt_payment: Option<String>,
    pub gmt_refund: Option<String>,
    pub gmt_close: Option<String>,

    pub fund_bill_list: Option<String>, // 原始 JSON 字符串，必要时再反序列化
    pub voucher_detail_list: Option<String>, // 同上
    pub biz_settle_mode: Option<String>,

    pub merchant_app_id: Option<String>,
    pub version: Option<String>,
}