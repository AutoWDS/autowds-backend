use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Clone, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "product_edition")]
pub enum ProductEdition {
    L0,
    L1,
    L2,
    L3,
}

#[derive(sqlx::Type, Clone, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "scheduler_type")]
pub enum SchedulerType {
    Browser,
    Fast,
}

#[derive(sqlx::Type, Clone, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "store_type")]
pub enum StoreType {
    MongoDB,
    RDB,
}

#[derive(sqlx::Type, Clone, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "task_status")]
pub enum TaskStatus {
    Cancelled,
    Failed,
    Running,
    Successful,
    Waiting,
}

#[derive(sqlx::Type, Clone, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "template_topic")]
pub enum TemplateTopic {
    Bidding,
    ECommerce,
    LocalLife,
    Media,
    Other,
    ResearchEducation,
    SearchEngine,
    SocialNetwork,
}
