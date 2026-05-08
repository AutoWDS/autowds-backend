pub use super::_entities::marketing_attribution::*;

use chrono::Local;
use sea_orm::{ActiveModelBehavior, ConnectionTrait, DbErr, Set};
use summer::async_trait;

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            let now = Local::now().naive_local();
            self.first_touch_at = Set(now);
            self.register_at = Set(now);
        }
        Ok(self)
    }
}
