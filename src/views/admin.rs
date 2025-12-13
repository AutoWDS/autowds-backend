use crate::model::{account_user, scraper_task, sea_orm_active_enums::ProductEdition, task_template};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ==================== 用户相关 ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListQuery {
    pub keyword: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResp {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub status: String,
    pub created_at: String,
}

impl From<account_user::Model> for UserResp {
    fn from(user: account_user::Model) -> Self {
        Self {
            id: user.id,
            username: user.name,
            email: user.email,
            status: if user.locked { "locked".to_string() } else { "active".to_string() },
            created_at: user.created.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserReq {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserReq {
    pub username: String,
    pub email: String,
    pub locked: Option<bool>,
    pub edition: Option<ProductEdition>,
}

// ==================== 任务相关 ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskListQuery {
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResp {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub template_id: i64,
    pub created_at: String,
    pub updated_at: String,
}

impl From<scraper_task::Model> for TaskResp {
    fn from(task: scraper_task::Model) -> Self {
        let status = if task.deleted {
            "completed"
        } else if task.data.is_some() {
            "running"
        } else {
            "pending"
        };

        Self {
            id: task.id,
            name: task.name,
            status: status.to_string(),
            template_id: 0, // 如果有template_id字段，需要从task中获取
            created_at: task.created.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: task.modified.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskReq {
    pub name: String,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskReq {
    pub name: String,
}

// ==================== 模板相关 ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateResp {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub config: Value,
    pub created_at: String,
}

impl From<task_template::Model> for TemplateResp {
    fn from(template: task_template::Model) -> Self {
        Self {
            id: template.id,
            name: template.name,
            description: template.detail,
            config: template.rule,
            created_at: template.created.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTemplateReq {
    pub name: String,
    pub description: Option<String>,
    pub config: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTemplateReq {
    pub name: String,
    pub description: Option<String>,
    pub config: Option<Value>,
}

// ==================== 统计相关 ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskStatisticsResp {
    pub pending: i64,
    pub running: i64,
    pub completed: i64,
    pub failed: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsOverviewResp {
    pub user_count: i64,
    pub task_count: i64,
    pub template_count: i64,
}
