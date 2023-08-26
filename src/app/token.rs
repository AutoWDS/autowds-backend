use std::net::IpAddr;

use crate::{
    model::user::AccountUser,
    utils::jwt::{self, Claims},
    AppState,
};
use actix_web::{error, post, web, HttpResponse, Scope};
use actix_web_validator::Json;
use serde::Deserialize;
use validator::Validate;

pub fn token_scope() -> Scope {
    return web::scope("/token").service(login);
}

#[post("")]
async fn login(
    state: web::Data<AppState>,
    ip_addr: Option<web::ReqData<IpAddr>>,
    body: Json<AuthenticationToken>,
) -> Result<HttpResponse, error::Error> {
    let mut rb = &state.rbatis;
    let user_optional = AccountUser::select_by_email(&mut rb, &body.email)
        .await
        .unwrap();

    let user = match user_optional {
        Some(u) => {
            if u.passwd != body.passwd {
                return Err(error::ErrorBadRequest("密码输入错误").into());
            }
            AccountUser {
                last_login: ip_addr.unwrap().into_inner(),
                // modified: TODO: modified;
                ..u
            }
        }
        None => return Err(error::ErrorNotFound("用户不存在，请先注册").into()),
    };

    let success = AccountUser::update_by_id(&mut rb, &user, user.id.unwrap())
        .await
        .unwrap();

    if success.rows_affected <= 0 {
        return Err(error::ErrorBadRequest("修改失败").into());
    }

    let claims = Claims::new(user.id.unwrap());
    return Ok(jwt::encode(claims)
        .map(|token| HttpResponse::Ok().json(token))
        .unwrap());
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthenticationToken {
    #[validate(email(message = "邮箱格式不正确"))]
    email: String,
    #[validate(length(min = 1, max = 30, message = "密码需在1-30个字符间"))]
    passwd: String,
}
