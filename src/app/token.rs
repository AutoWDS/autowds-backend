use actix_web::http::header::ContentType;
use actix_web::{error, post, web, HttpResponse, Scope};
use actix_web_validator::Json;
use chrono::Local;
use ormlitex::model::*;
use serde::Deserialize;
use std::net::IpAddr;
use validator::Validate;

use crate::http::error::Result;
use crate::model::account_user::AccountUser;
use crate::utils::jwt::{self, Claims};
use crate::AppState;

pub fn token_scope() -> Scope {
    return web::scope("/token").service(login);
}

#[post("")]
async fn login(
    state: web::Data<AppState>,
    ip_addr: Option<web::ReqData<IpAddr>>,
    body: Json<AuthenticationToken>,
) -> Result<HttpResponse> {
    let db = &state.db;
    let user_optional = AccountUser::select_by_email(&db, &body.email).await?;

    let user = match user_optional {
        None => return Err(error::ErrorNotFound("用户不存在，请先注册").into()),
        Some(u) => {
            if u.passwd != body.passwd {
                return Err(error::ErrorBadRequest("密码输入错误").into());
            }
            u.update_partial()
                .modified(Local::now().naive_local())
                .last_login(ip_addr.unwrap().into_inner())
                .update(db)
                .await?
        }
    };

    let claims = Claims::new(user.id);
    return Ok(jwt::encode(claims).map(|token| {
        HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body(token)
    })?);
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthenticationToken {
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,
    #[validate(length(min = 1, max = 30, message = "密码需在1-30个字符间"))]
    pub passwd: String,
}
