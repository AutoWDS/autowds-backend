use chrono::NaiveDateTime;
use ormlite::model::*;

#[derive(Debug, Model)]
#[ormlite(table = "favorite")]
pub struct Favorite {
    #[ormlite(primary_key)]
    pub id: i64,
    pub created: NaiveDateTime,
    pub user_id: i64,
    pub template_id: i64,
}
