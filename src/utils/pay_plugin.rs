use crate::config::pay::PayConfig;
use alipay_sdk_rust::pay::{PayClient, Payer};
use derive_more::derive::Deref;
use spring::{
    app::AppBuilder,
    async_trait,
    config::ConfigRegistry,
    plugin::{MutableComponentRegistry, Plugin},
};
use std::sync::Arc;
use wechat_pay_rust_sdk::pay::WechatPay;

pub struct PayPlugin;

#[async_trait]
impl Plugin for PayPlugin {
    async fn build(&self, app: &mut AppBuilder) {
        let conf = app.get_config::<PayConfig>().expect("支付配置获取失败");

        if conf.alipay_enable {
            let alipay = PayClient::builder()
                .api_url(&conf.alipay_api_url)
                .app_id(&conf.alipay_app_id)
                .alipay_root_cert_sn(&conf.alipay_root_cert_sn)
                .alipay_public_key(&conf.alipay_public_key)
                .app_cert_sn(&conf.alipay_app_cert_sn)
                .notify_url(&conf.alipay_callback_url)
                .charset_utf8()
                .format_json()
                .private_key(&conf.alipay_app_private_key)
                .public_key(&conf.alipay_app_public_key)
                .sign_type_rsa2()
                .version_1_0()
                .build()
                .expect("build alipay client failed");

            app.add_component(Alipay(Arc::new(alipay)));
        }

        if conf.wechat_pay_enable {
            let wechat_pay = WechatPay::from_env();
            app.add_component(WechatPayClient(Arc::new(wechat_pay)));
        }
    }
}

#[derive(Clone, Deref)]
pub struct Alipay(Arc<dyn Payer + Send + Sync>);

#[derive(Clone, Deref)]
pub struct WechatPayClient(Arc<WechatPay>);