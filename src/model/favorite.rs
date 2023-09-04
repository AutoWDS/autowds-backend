use chrono::NaiveDateTime;
use ormlitex::model::*;

#[derive(Debug, Model)]
#[ormlitex(table = "favorite")]
pub struct Favorite {
    #[ormlitex(primary_key)]
    pub id: i64,
    pub created: NaiveDateTime,
    pub user_id: i64,
    pub template_id: i64,
}
