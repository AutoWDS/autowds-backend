use anyhow::Context as _;
use aws_config::BehaviorVersion;
use aws_credential_types::Credentials;
use aws_sdk_s3::{
    config::{Builder as S3ConfigBuilder, Region},
    Client,
};
use summer::{
    app::AppBuilder,
    async_trait,
    config::ConfigRegistry,
    plugin::{MutableComponentRegistry, Plugin},
};

use crate::config::s3::TaskLogS3Config;

pub struct S3Plugin;

#[async_trait]
impl Plugin for S3Plugin {
    async fn build(&self, app: &mut AppBuilder) {
        let config = app
            .get_config::<TaskLogS3Config>()
            .expect("读取 [s3] 配置失败");

        let client = if config.is_configured() {
            Some(S3ClientInner {
                client: build_s3_client(&config).await,
                bucket: config.bucket.trim().to_string(),
                prefix: config.prefix.trim().to_string(),
            })
        } else {
            None
        };
        app.add_component(S3Client(client));
    }
}

#[derive(Clone)]
pub struct S3Client(Option<S3ClientInner>);

#[derive(Clone)]
struct S3ClientInner {
    client: Client,
    bucket: String,
    prefix: String,
}

impl S3Client {
    pub fn is_configured(&self) -> bool {
        self.0.is_some()
    }

    pub async fn get_object_bytes(&self, object_key: &str) -> anyhow::Result<Vec<u8>> {
        let Some(inner) = self.0.as_ref() else {
            anyhow::bail!("服务端 [s3] 未配置完整");
        };

        let key = inner.object_key(object_key);
        let resp = inner
            .client
            .get_object()
            .bucket(&inner.bucket)
            .key(&key)
            .send()
            .await
            .with_context(|| format!("S3 GetObject bucket={} key={}", inner.bucket, key))?;

        let body = resp
            .body
            .collect()
            .await
            .with_context(|| format!("读取 S3 对象 body key={}", key))?;

        Ok(body.into_bytes().to_vec())
    }
}

impl S3ClientInner {
    /// `object_key` 为库中存的相对键；若配置了 `prefix`，则拼到对象完整 key。
    fn object_key(&self, object_key: &str) -> String {
        let object_key = object_key.trim().trim_start_matches('/');
        if self.prefix.is_empty() {
            object_key.to_string()
        } else {
            format!("{}/{}", self.prefix.trim_end_matches('/'), object_key)
        }
    }
}

async fn build_s3_client(config: &TaskLogS3Config) -> Client {
    let creds = Credentials::new(
        config.access_key_id.trim(),
        config.secret_access_key.trim(),
        None,
        None,
        "autowds-backend-s3",
    );

    let sdk = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(config.region.trim().to_string()))
        .credentials_provider(creds)
        .endpoint_url(config.endpoint.trim())
        .load()
        .await;

    let s3_conf = S3ConfigBuilder::from(&sdk).force_path_style(true).build();
    Client::from_conf(s3_conf)
}
