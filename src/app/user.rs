use actix_web::{error, get, patch, post, web, HttpResponse, Responder, Scope};
use actix_web_validator::Json;
use chrono::Local;
use deadpool_redis::redis;
use ormlitex::model::*;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use utoipa::{OpenApi, ToSchema};
use validator::Validate;

use crate::http::error::Result;
use crate::model::account_user::{AccountUser, NewAccountUser};
use crate::model::enums::ProductEdition;
use crate::utils::jwt::Claims;
use crate::utils::mail::{self, ValidateCodeMailTemplate};
use crate::utils::rand;
use crate::AppState;

#[derive(OpenApi)]
#[openapi(
    tags((name = "User", description = "用户相关服务")),
    paths(register, reset_passwd),
    components(schemas(RegisterDTO,ResetPasswdDTO,SendEmailDTO,SetNameReqDto,AccountUser))
)]
pub struct UserDoc;

pub fn user_scope() -> Scope {
    return web::scope("/user")
        .service(register)
        .service(reset_passwd)
        .service(register_validate_code)
        .service(reset_validate_code)
        .service(get_current_user)
        .service(rename);
}

#[utoipa::path(
    context_path = "/user",
    request_body=RegisterDTO,
    responses(
        (status = 200, description = "注册", body = [AccountUser])
    )
)]
#[post("")]
async fn register(
    state: web::Data<AppState>,
    ip_addr: Option<web::ReqData<IpAddr>>,
    body: Json<RegisterDTO>,
) -> Result<HttpResponse> {
    let db = &state.db;
    let exists = AccountUser::exists_by_email(&db, body.email.as_str()).await?;

    if exists {
        return Err(error::ErrorBadRequest("邮箱已被注册").into());
    }

    let user = NewAccountUser {
        locked: false,
        edition: ProductEdition::L0,
        email: String::from(&body.email),
        name: String::from(&body.name),
        passwd: String::from(&body.passwd),
        last_login: ip_addr.unwrap().into_inner(),
        created: Local::now().naive_local(),
        modified: Local::now().naive_local(),
    }
    .insert(db)
    .await?;

    return Ok(HttpResponse::Ok().json(user));
}

#[utoipa::path(
    context_path = "/user",
    request_body=ResetPasswdDTO,
    responses(
        (status = 200, description = "改密码", body = [AccountUser])
    )
)]
#[post("/passwd")]
async fn reset_passwd(
    state: web::Data<AppState>,
    ip_addr: Option<web::ReqData<IpAddr>>,
    body: Json<ResetPasswdDTO>,
) -> Result<HttpResponse> {
    let db = &state.db;
    let user_optional = AccountUser::select_by_email(&db, &body.email).await?;

    let user = match user_optional {
        None => return Err(error::ErrorNotFound("用户不存在").into()),
        Some(u) => {
            u.update_partial()
                .passwd(&body.passwd)
                .modified(Local::now().naive_local())
                .last_login(ip_addr.unwrap().into_inner())
                .update(db)
                .await?
        }
    };

    return Ok(HttpResponse::Ok().json(user));
}

#[post("/register-validate-code")]
async fn register_validate_code(
    state: web::Data<AppState>,
    body: Json<SendEmailDTO>,
) -> Result<impl Responder> {
    let validate_code = generate_validate_code(&state, &body.email).await?;
    let template = ValidateCodeMailTemplate {
        tip: "欢迎您注册我们的服务，您的注册验证码(5分钟内有效)是：",
        validate_code: validate_code.as_str(),
    };
    mail::send_mail(&body.email, "注册验证码", &template);
    Ok("")
}

#[post("/reset-validate-code")]
async fn reset_validate_code(
    state: web::Data<AppState>,
    body: Json<SendEmailDTO>,
) -> Result<impl Responder> {
    let validate_code = generate_validate_code(&state, &body.email).await?;
    let template = ValidateCodeMailTemplate {
        tip: "请确认您是否需要重置密码，重置密码请在系统中输入以下验证码(5分钟内有效)：",
        validate_code: validate_code.as_str(),
    };
    mail::send_mail(&body.email, "重置密码的验证码", &template);
    Ok("")
}

async fn generate_validate_code(state: &AppState, email: &str) -> Result<String> {
    let rand_code = rand::rand_alphanumeric(6);
    let mut conn = state.redis.get().await?;
    redis::cmd("SETEX")
        .arg(format!("email-validate:{}", email))
        .arg(300) // 5min expire
        .arg(&rand_code)
        .query_async::<_, ()>(&mut conn)
        .await?;
    return Ok(rand_code);
}

#[get("")]
async fn get_current_user(state: web::Data<AppState>, claims: Claims) -> Result<HttpResponse> {
    let user = AccountUser::fetch_one(claims.uid, &state.db).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[patch("/name")]
async fn rename(
    state: web::Data<AppState>,
    claims: Claims,
    body: Json<SetNameReqDto>,
) -> Result<HttpResponse> {
    let success = AccountUser::update_name(&state.db, claims.uid, &body.name).await?;
    Ok(HttpResponse::Ok().json(success))
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterDTO {
    #[validate(length(min = 1, max = 30, message = "用户名需在1-30个字符间"))]
    pub name: String,
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,
    #[validate(length(min = 1, max = 30, message = "密码需在1-30个字符间"))]
    pub passwd: String,
    #[serde(rename = "validateCode")]
    pub validate_code: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ResetPasswdDTO {
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,
    #[validate(length(min = 1, max = 30, message = "密码需在1-30个字符间"))]
    pub passwd: String,
    #[serde(rename = "validateCode")]
    pub validate_code: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendEmailDTO {
    #[validate(email(message = "邮箱格式不正确"))]
    email: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetNameReqDto {
    #[validate(length(min = 1, max = 30, message = "用户名需在1-30个字符间"))]
    name: String,
}
