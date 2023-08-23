#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScraperTask {
    id: i64,
    userId: i64,
    name: String,
    deleted: bool,
    rule: String,
    created: String,
    modified: String,
}

crud!(ScraperTask {});

impl_select!(ScraperTask{select_by_email(email:&str)->Option => "`where email=#{email}`"});

impl_update!(ScraperTask{update_by_id(id:i64)=>"`where id = 1`"});

#[sql("select count(*)>0 from account_user where email=#{email}")]
pub async fn exists_by_email(rb: &RBatis, email: &str) -> bool {}
