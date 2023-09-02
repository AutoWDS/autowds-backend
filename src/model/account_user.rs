use std::net::IpAddr;

use super::enums::ProductEdition;
use chrono::{Local, NaiveDateTime};
use ormlite::{model::*, Result};
use serde::Serialize;
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Clone, Debug, Model, Serialize, ToSchema)]
#[ormlite(table = "account_user")]
pub struct AccountUser {
    #[ormlite(primary_key)]
    pub id: Option<i64>,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub edition: ProductEdition,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub passwd: String,
    pub locked: bool,
    #[serde(skip_serializing)]
    pub last_login: IpAddr,
}

impl AccountUser {
    pub async fn exists_by_email(db: &PgPool, email: &str) -> Result<bool> {
        let (exists,): (bool,) =
            ormlite::query_as("SELECT count(*)>0 FROM account_user where email=$1")
                .bind(email)
                .fetch_one(db)
                .await?;
        Ok(exists)
    }

    pub async fn select_by_email(db: &PgPool, email: &str) -> Result<Option<AccountUser>> {
        AccountUser::select()
            .where_("email=?")
            .bind(email)
            .fetch_optional(db)
            .await
    }

    pub async fn update_name(db: &PgPool, uid: i64, name: &str) -> Result<bool> {
        let effect = sqlx::query("update account_user set name=?,modified=? where id=?")
            .bind(name)
            .bind(Local::now().naive_local())
            .bind(uid)
            .execute(db)
            .await?
            .rows_affected();
        Ok(effect > 0)
    }
}
