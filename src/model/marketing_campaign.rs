pub use super::_entities::marketing_campaign::*;

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
            self.created_at = Set(Local::now().naive_local());
            if self.status.is_not_set() {
                self.status = Set("draft".to_string());
            }
        }
        self.modified_at = Set(Local::now().naive_local());
        Ok(self)
    }
}
