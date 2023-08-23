use user::ProductEdition;

#[derive(Debug, Deserialize, Serialize)]
pub enum TemplateTopic {
    SOCIAL_NETWORK = "社交网络",
    RESEARCH_EDUCATION = "科研教育",
    E_COMMERCE = "电子商务",
    LOCAL_LIFE = "本地生活",
    BIDDING = "招投标",
    MEDIA = "媒体阅读",
    SEARCH_ENGINE = "搜索引擎",
    OTHER = "其他类型",
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskTemplate {
    id: i64,
    name: String,
    detail: String,
    img: String,
    topic: TemplateTopic,
    edition: ProductEdition,
    lang: String,
    fav_count: u32,
    rule: String,
    params: String,
    data: String,
    created: String,
    modified: String,
}

crud!(TaskTemplate {});

impl_select!(TaskTemplate{select_by_email(email:&str)->Option => "`where email=#{email}`"});

impl_update!(TaskTemplate{update_by_id(id:i64)=>"`where id = 1`"});

#[sql("select count(*)>0 from account_user where email=#{email}")]
pub async fn exists_by_email(rb: &RBatis, email: &str) -> bool {}
