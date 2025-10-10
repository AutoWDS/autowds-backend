mod task;
mod template;
mod token;
mod user;

use axum_client_ip::ClientIpSource;
use spring::config::env::Env;
use spring_web::{
    aide::OperationInput,
    axum::{
        body,
        http::request::Parts,
        middleware::{self, Next},
        response::{IntoResponse, Response},
    },
    extractor::{FromRequestParts, Request},
    Router,
};

pub fn router() -> Router {
    let env = Env::init();
    spring_web::handler::auto_router()
        .layer(middleware::from_fn(problem_middleware))
        .layer(match env {
            Env::Dev => ClientIpSource::ConnectInfo.into_extension(),
            _ => ClientIpSource::RightmostXForwardedFor.into_extension(),
        })
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
