pub use super::_entities::credit_log::*;

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
            self.created = Set(Local::now().naive_local());
        }
        Ok(self)
    }
}