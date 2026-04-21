pub use _entities::prelude;
pub use _entities::sea_orm_active_enums;

mod _entities;

pub mod account_user;
pub mod credit_log;
pub mod favorite;
pub mod pay_order;
pub mod scraper_task;
pub mod task_template;

impl sea_orm_active_enums::ProductEdition {
    /// 获取当前版本的任务数量上限
    pub fn task_limit(&self) -> u64 {
        match self {
            Self::L0 => 5,
            Self::L1 => 100,
            Self::L2 => u64::MAX,
            Self::L3 => u64::MAX,
        }
    }
}
