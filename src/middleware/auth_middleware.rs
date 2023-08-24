use actix_web::body::EitherBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::Method;
use actix_web::http::{self, header};
use actix_web::{Error, HttpMessage};
use actix_web::{HttpResponse, Responder};
use futures_util::future::LocalBoxFuture;
use glob::Pattern;
use std::future::{ready, Ready};

use crate::utils::jwt;

use super::problemdetails;

lazy_static! {
    static ref IGNORE_ROUTES: [Pattern; 2] =
        ["/template", "/instance/*/log"].map(|path| Pattern::new(path).unwrap());
}

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
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
        let mut authenticate_pass: bool = false;

        if Method::OPTIONS == *req.method() {
            // 忽略OPTIONS请求
            authenticate_pass = true;
        } else {
            // 忽略某些路径的请求
            for ignore_route in IGNORE_ROUTES.iter() {
                if ignore_route.matches(req.path()) {
                    authenticate_pass = true;
                }
            }
        }

        if !authenticate_pass {
            // 没有忽略的请求，需要对jwt进行解码
            if let Some(auth_token) = req.headers().get(header::AUTHORIZATION) {
                let token = auth_token.to_str().unwrap();
                if let Ok(claims) = jwt::decode(token) {
                    // 解码成功
                    req.extensions_mut().insert(claims);
                } else {
                    authenticate_pass = false;
                }
            } else {
                authenticate_pass = false;
            }
        }

        if !authenticate_pass {
            let (request, _pl) = req.into_parts();
            let pd = problemdetails::Problem::from(http::StatusCode::UNAUTHORIZED)
                .with_instance(request.uri().to_string())
                .with_detail("token error");
            let response = HttpResponse::Unauthorized().json(pd).map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(req);

        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}
