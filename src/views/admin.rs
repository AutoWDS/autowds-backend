use crate::model::{account_user, scraper_task, sea_orm_active_enums::{ProductEdition, TemplateTopic}, task_template};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

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
    pub credits: i32,
    pub invite_code: String,
    pub invited_by: Option<i64>,
    pub edition: ProductEdition,
}

impl From<account_user::Model> for UserResp {
    fn from(user: account_user::Model) -> Self {
        Self {
            id: user.id,
            username: user.name,
            email: user.email,
            status: if user.locked { "locked".to_string() } else { "active".to_string() },
            created_at: user.created.format("%Y-%m-%d %H:%M:%S").to_string(),
            credits: user.credits,
            invite_code: user.invite_code,
            invited_by: user.invited_by,
            edition: user.edition,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserReq {
    #[validate(length(min = 1, max = 32, message = "用户名长度必须在1-32字符之间"))]
    pub username: String,
    #[validate(email(message = "邮箱格式不正确"), length(max = 64, message = "邮箱长度不能超过64字符"))]
    pub email: String,
    #[validate(length(min = 6, max = 32, message = "密码长度必须在6-32字符之间"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserReq {
    #[validate(length(min = 1, max = 32, message = "用户名长度必须在1-32字符之间"))]
    pub username: String,
    #[validate(email(message = "邮箱格式不正确"), length(max = 64, message = "邮箱长度不能超过64字符"))]
    pub email: String,
    pub locked: Option<bool>,
    pub edition: Option<ProductEdition>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AdjustCreditsReq {
    #[validate(range(min = -10000, max = 10000, message = "积分调整范围必须在-10000到10000之间"))]
    pub amount: i32,
    #[validate(length(min = 1, max = 200, message = "描述长度必须在1-200字符之间"))]
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserEditionReq {
    pub edition: ProductEdition,
    #[validate(length(min = 1, max = 200, message = "描述长度必须在1-200字符之间"))]
    pub description: String,
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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTaskReq {
    #[validate(length(min = 1, max = 60, message = "任务名称长度必须在1-60字符之间"))]
    pub name: String,
    #[validate(range(min = 1, message = "用户ID必须大于0"))]
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTaskReq {
    #[validate(length(min = 1, max = 60, message = "任务名称长度必须在1-60字符之间"))]
    pub name: String,
}

// ==================== 模板相关 ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateResp {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub rule: Value,
    pub topic: TemplateTopic,
    pub edition: ProductEdition,
    pub img: String,
    pub lang: String,
    pub params: Option<Value>,
    pub created_at: String,
}

impl From<task_template::Model> for TemplateResp {
    fn from(template: task_template::Model) -> Self {
        Self {
            id: template.id,
            name: template.name,
            description: template.detail,
            rule: template.rule,
            topic: template.topic,
            edition: template.edition,
            img: template.img,
            lang: template.lang,
            params: template.params,
            created_at: template.created.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTemplateReq {
    #[validate(length(min = 1, max = 80, message = "模板名称长度必须在1-80字符之间"))]
    pub name: String,
    #[validate(length(max = 200, message = "描述长度不能超过200字符"))]
    pub description: Option<String>,
    pub rule: Option<Value>,
    pub topic: TemplateTopic,
    pub edition: ProductEdition,
    #[validate(length(max = 200, message = "图片URL长度不能超过200字符"))]
    pub img: Option<String>,
    #[validate(length(min = 2, max = 6, message = "语言代码长度必须在2-6字符之间"))]
    pub lang: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTemplateReq {
    #[validate(length(min = 1, max = 80, message = "模板名称长度必须在1-80字符之间"))]
    pub name: String,
    #[validate(length(max = 200, message = "描述长度不能超过200字符"))]
    pub description: Option<String>,
    pub rule: Option<Value>,
    pub topic: TemplateTopic,
    pub edition: ProductEdition,
    #[validate(length(max = 200, message = "图片URL长度不能超过200字符"))]
    pub img: Option<String>,
    #[validate(length(min = 2, max = 6, message = "语言代码长度必须在2-6字符之间"))]
    pub lang: String,
    pub params: Option<Value>,
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
