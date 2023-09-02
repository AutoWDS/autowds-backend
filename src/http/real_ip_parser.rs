use actix_web::body::EitherBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpRequest};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::net::IpAddr;

pub struct RealIP;

impl<S, B> Transform<S, ServiceRequest> for RealIP
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RealIPMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RealIPMiddleware { service }))
    }
}

pub struct RealIPMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RealIPMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let ip_addr = get_real_ip(req.request());
        req.extensions_mut().insert(ip_addr);

        let res = self.service.call(req);
        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}

fn get_real_ip(req: &HttpRequest) -> IpAddr {
    let conn = req.connection_info();
    let ip_addr = if let Some(x_real_ip) = req.headers().get("X-Real-IP") {
        if !x_real_ip.is_empty() {
            x_real_ip.to_str().unwrap()
        } else {
            conn.realip_remote_addr().unwrap()
        }
    } else {
        conn.realip_remote_addr().unwrap()
    };
    return ip_addr.parse().unwrap();
}
