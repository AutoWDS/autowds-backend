use actix_web::{dev, middleware::ErrorHandlerResponse, Result};
use reqwest::header;

use super::problemdetails;

pub fn handle_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let (req, res) = res.into_parts();
    let pd = problemdetails::Problem::from(res.status())
        .with_instance(req.uri().to_string())
        .with_detail(match res.error() {
            Some(e) => e.to_string(),
            None => "".into(),
        });
    let mut res = res
        .set_body(serde_json::to_string(&pd).unwrap())
        .map_into_boxed_body();

    res.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/problem+json"),
    );

    let res = dev::ServiceResponse::new(req, res).map_into_right_body();
    Ok(ErrorHandlerResponse::Response(res))
}
