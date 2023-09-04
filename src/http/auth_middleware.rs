use actix_web::body::EitherBody;
use actix_web::dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use actix_web::{error, Error, HttpMessage, HttpRequest};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

use crate::utils::jwt::{self, Claims};

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
        // 对jwt进行解码
        if let Some(auth_token) = req.headers().get(header::AUTHORIZATION) {
            let token = auth_token.to_str().unwrap();
            match jwt::decode(token) {
                Ok(claims) => {
                    req.extensions_mut().insert(claims);
                }
                Err(e) => {
                    return Box::pin(async {
                        Ok(req
                            .error_response(error::ErrorUnauthorized(e))
                            .map_into_right_body())
                    });
                }
            }
        }

        let res = self.service.call(req);

        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}

impl actix_web::FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(match req.extensions().get::<Claims>() {
            Some(claims) => Ok(claims.clone()),
            None => Err(error::ErrorUnauthorized("请先登录").into()),
        })
    }
}
