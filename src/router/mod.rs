mod admin;
mod pay;
mod pay_query;
mod statistics;
mod task;
mod template;
mod token;
mod user;

use axum_client_ip::ClientIpSource;
use summer::config::env::Env;
use summer_web::{
    aide::OperationInput,
    axum::{
        body,
        http::request::Parts,
        middleware::{self, Next},
        response::{IntoResponse, Response},
    },
    extractor::{FromRequestParts, Request},
    middleware::services::{ServeDir, ServeFile},
    Router,
};

pub fn router() -> Router {
    let env = Env::init();
    let router = Router::new().nest(
        "/api",
        summer_web::handler::auto_router()
            .nest("/pay", pay::router().into())
            .layer(middleware::from_fn(problem_middleware))
            .layer(match env {
                Env::Dev => ClientIpSource::ConnectInfo.into_extension(),
                _ => ClientIpSource::RightmostXForwardedFor.into_extension(),
            }),
    );
    match env {
        Env::Dev => {
            let home_dir =
                ServeDir::new("site/out").not_found_service(ServeFile::new("site/out/index.html"));
            let cloud_dir = ServeDir::new("frontend/build")
                .not_found_service(ServeFile::new("frontend/build/index.html"));
            let admin_dir = ServeDir::new("backend/dist")
                .not_found_service(ServeFile::new("backend/dist/index.html"));
            router
                .nest_service("/cloud/", cloud_dir)
                .nest_service("/admin/", admin_dir)
                .fallback_service(home_dir)
        }
        _ => {
            // 看Dockerfile最终构建所放置的路径
            let home_dir =
                ServeDir::new("static").not_found_service(ServeFile::new("static/index.html"));
            let cloud_dir = ServeDir::new("static/cloud")
                .not_found_service(ServeFile::new("static/cloud/index.html"));
            let admin_dir = ServeDir::new("static/admin")
                .not_found_service(ServeFile::new("static/admin/index.html"));
            router
                .nest_service("/cloud/", cloud_dir)
                .nest_service("/admin/", admin_dir)
                .fallback_service(home_dir)
        }
    }
}

async fn problem_middleware(request: Request, next: Next) -> Response {
    let uri = request.uri().path().to_string();
    let response = next.run(request).await;
    let status = response.status();
    if status.is_client_error() || status.is_server_error() {
        let msg = response.into_body();
        let msg = body::to_bytes(msg, usize::MAX)
            .await
            .expect("server body read failed");
        let msg = String::from_utf8(msg.to_vec()).expect("read body to string failed");
        problemdetails::new(status)
            .with_instance(uri)
            .with_title(status.canonical_reason().unwrap_or("error"))
            .with_detail(msg)
            .into_response()
    } else {
        response
    }
}

#[derive(Debug)]
pub struct ClientIp(axum_client_ip::ClientIp);

impl OperationInput for ClientIp {}

impl<S> FromRequestParts<S> for ClientIp
where
    S: Sync,
{
    type Rejection = axum_client_ip::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self(
            axum_client_ip::ClientIp::from_request_parts(parts, state).await?,
        ))
    }
}
