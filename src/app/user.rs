use actix_web::{error, post, web, HttpResponse, Scope};
use actix_web_validator::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::app::AppState;
use crate::model::user::AccountUser;
use crate::utils::mail::{self, ValidateCodeMailTemplate};
use crate::utils::rand;

pub fn user_scope() -> Scope {
    return web::scope("/user")
        .service(register)
        .service(reset_passwd)
        .service(register_validate_code)
        .service(reset_validate_code);
}

#[post("/")]
async fn register(
    state: web::Data<AppState>,
    body: Json<RegisterDTO>,
) -> Result<HttpResponse, error::Error> {
    let (exists,): (bool,) = sqlx::query_as("select count(*)>0 from account_user where email=$1")
        .bind(&body.email)
        .fetch_one(&state.pg_pool)
        .await
        .unwrap();

    if exists {
        return Err(error::ErrorBadRequest("邮箱已被注册").into());
    }

    let success = sqlx::query("insert into account_user ()")
        .bind(&body.email)
        .execute(&state.pg_pool)
        .await
        .unwrap();

    if success.rows_affected() <= 0 {
        return Err(error::ErrorInternalServerError("注册失败").into());
    }

    return Ok(HttpResponse::Ok().json("注册成功"));
}

#[post("/passwd")]
async fn reset_passwd(
    state: web::Data<AppState>,
    body: Json<ResetPasswdDTO>,
) -> Result<HttpResponse, error::Error> {
    let user_optional: Option<AccountUser> =
        sqlx::query_as::<_, AccountUser>("select * from account_user where email=$1")
            .bind(&body.email)
            .fetch_optional(&state.pg_pool)
            .await
            .unwrap();

    let user = match user_optional {
        Some(u) => u,
        None => return Err(error::ErrorNotFound("用户不存在").into()),
    };

    let success = sqlx::query("update account_user set passwd=$1 where id=$2")
        .bind(&body.passwd)
        .bind(user.id)
        .execute(&state.pg_pool)
        .await
        .unwrap();

    if success.rows_affected() <= 0 {
        return Err(error::ErrorBadRequest("修改失败").into());
    }

    return Ok(HttpResponse::Ok().json(user));
}

#[post("/register-validate-code")]
async fn register_validate_code(body: Json<SendEmailDTO>) -> &'static str {
    let template = ValidateCodeMailTemplate {
        tip: "欢迎您注册我们的服务，您的注册验证码(5分钟内有效)是：",
        validate_code: "",
    };
    mail::send_mail(&body.email, "注册验证码", &template);
    "ok"
}

#[post("/reset-validate-code")]
async fn reset_validate_code(body: Json<SendEmailDTO>) -> &'static str {
    let rand_code = rand::rand_alphanumeric(6);
    let template = ValidateCodeMailTemplate {
        tip: "请确认您是否需要重置密码，重置密码请在系统中输入以下验证码(5分钟内有效)：",
        validate_code: rand_code.as_str(),
    };
    mail::send_mail(&body.email, "重置密码的验证码", &template);
    "ok"
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct RegisterDTO {
    #[validate(length(min = 1, max = 30, message = "用户名需在1-30个字符间"))]
    name: String,
    #[validate(email(message = "邮箱格式不正确"))]
    email: String,
    #[validate(length(min = 1, max = 30, message = "密码需在1-30个字符间"))]
    passwd: String,
    #[serde(rename = "validateCode")]
    validate_code: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct ResetPasswdDTO {
    #[validate(email(message = "邮箱格式不正确"))]
    email: String,
    #[validate(length(min = 1, max = 30, message = "密码需在1-30个字符间"))]
    passwd: String,
    #[serde(rename = "validateCode")]
    validate_code: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
struct SendEmailDTO {
    #[validate(email(message = "邮箱格式不正确"))]
    email: String,
}
