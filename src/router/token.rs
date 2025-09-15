use crate::{
    model::{account_user, prelude::AccountUser},
    utils::jwt::{self, Claims},
    views::user::AuthenticationToken,
};
use anyhow::Context as _;
use sea_orm::{ColumnTrait as _, EntityTrait as _, QueryFilter as _};
use serde_json::json;
use spring_sea_orm::DbConn;
use spring_web::{axum::response::IntoResponse, post};
use spring_web::{
    axum::Json,
    error::{KnownWebError, Result},
    extractor::Component,
};

#[post("/token")]
async fn login(
    Component(db): Component<DbConn>,
    Json(body): Json<AuthenticationToken>,
) -> Result<impl IntoResponse> {
    let user = AccountUser::find()
        .filter(account_user::Column::Email.eq(&body.email))
        .one(&db)
        .await
        .context("query db failed")?
        .ok_or_else(|| KnownWebError::unauthorized("用户不存在，请先注册"))?;

    if user.passwd != body.passwd {
        Err(KnownWebError::unauthorized("密码错误"))?;
    }

    let claims = Claims::new(user.clone());
    let token = jwt::encode(claims)?;
    Ok(Json(json!({
        "id": user.id,
        "name": user.name,
        "email": user.email,
        "edition": user.edition,
        "token": token,
    })))
}
