use actix_web::{dev::Payload, Error, HttpRequest};
use futures::future::{ready, Ready};
use std::pin::Pin;

// 自定义提取器类型
struct MyCustomExtractor;

impl MyCustomExtractor {
    fn new() -> Self {
        MyCustomExtractor
    }
}

impl
    actix_service::Service<
        Request = dev::ServiceRequest,
        Response = dev::ServiceResponse,
        Error = Error,
    > for MyCustomExtractor
{
    type Future = Ready<Result<Self::Response, Self::Error>>;
    type Response = dev::ServiceResponse;
    type Error = Error;

    fn poll_ready(
        &self,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&self, req: dev::ServiceRequest) -> Self::Future {
        let header_value = req.headers().get("X-Custom-Header");

        match header_value {
            Some(value) => {
                let value_str = value.to_str().unwrap_or_default();
                // 在这里可以对提取的数据进行进一步处理
                let extracted_data = value_str.to_string();
                // 调用处理程序函数，将提取的数据传递给它
                let response = req
                    .clone()
                    .into_response(HttpResponse::Ok().body(extracted_data));
                ready(Ok(response))
            }
            None => {
                // 如果找不到头部信息，则返回错误响应
                let response = req.error_response(HttpResponse::BadRequest());
                ready(Err(ErrorInternalServerError(response)))
            }
        }
    }
}

// 实现FromRequest trait来使用自定义提取器
impl actix_web::FromRequest for MyCustomExtractor {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(Ok(MyCustomExtractor::new()))
    }
}
