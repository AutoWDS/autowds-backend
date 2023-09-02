use super::enums::SchedulerType;
use chrono::NaiveDateTime;
use ormlite::model::*;

#[derive(Debug, Model)]
#[ormlite(table = "schedule_config")]
pub struct ScheduleConfig {
    #[ormlite(primary_key)]
    pub id: i64,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub next_time: NaiveDateTime,
    pub user_id: i64,
    pub store_id: i64,
    pub proxy_id: i64,
    #[ormlite(column = "type")]
    pub stype: SchedulerType,
    pub finished: bool,
    pub cron: String,
}
