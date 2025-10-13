use crate::model::{account_user, sea_orm_active_enums::ProductEdition};
use askama::Template;
use schemars::JsonSchema;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// # 认证请求
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AuthenticationToken {
    pub email: String,
    pub passwd: String,
}

/// # 注册请求
#[derive(Debug, Validate, Deserialize, JsonSchema)]
pub struct RegisterReq {
    /// # 用户名
    #[validate(length(max = 30, message = "用户名不能超过30个字符"))]
    pub name: String,

    /// # 邮箱
    #[validate(
        email(message = "邮箱格式不正确"),
        length(max = 60, message = "邮箱过长")
    )]
    pub email: String,

    /// # 密码
    #[validate(length(max = 32, message = "密码过长"))]
    pub passwd: String,

    /// # 验证码
    #[validate(length(max = 8, message = "验证码过长"))]
    pub validate_code: String,
}

#[derive(Debug, Validate, Deserialize, JsonSchema)]
pub struct ResetPasswdReq {
    #[validate(
        email(message = "邮箱格式不正确"),
        length(max = 60, message = "邮箱过长")
    )]
    pub email: String,
    #[validate(length(max = 32, message = "密码过长"))]
    pub passwd: String,
    #[validate(length(max = 8, message = "验证码过长"))]
    pub validate_code: String,
}

#[derive(Debug, Validate, Deserialize, JsonSchema)]
pub struct SendEmailReq {
    #[validate(
        email(message = "邮箱格式不正确"),
        length(max = 60, message = "邮箱过长")
    )]
    pub email: String,
}

#[derive(Debug, Validate, Deserialize, JsonSchema)]
pub struct SetNameReq {
    #[validate(length(max = 30, message = "用户名不能超过30个字符"))]
    pub name: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct UserResp {
    pub id: i64,
    pub created: DateTime,
    pub modified: DateTime,
    pub edition: ProductEdition,
    pub name: String,
    pub email: String,
    pub locked: bool,
    pub last_login: Option<String>,
}

impl From<account_user::Model> for UserResp {
    fn from(user: account_user::Model) -> Self {
        Self {
            id: user.id,
            created: user.created,
            modified: user.modified,
            edition: user.edition,
            name: user.name,
            email: user.email,
            locked: user.locked,
            last_login: user.last_login.map(|ip| ip.to_string()),
        }
    }
}

#[derive(Template)]
#[template(path = "mail/validate_code.html")]
pub struct ValidateCodeEmailTemplate<'a> {
    /// # 提示
    pub tip: &'a str,
    /// # 验证码
    pub code: &'a str,
}
