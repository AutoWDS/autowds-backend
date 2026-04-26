pub use super::_entities::task_template::*;

use chrono::Local;
use sea_orm::{ActiveModelBehavior, ConnectionTrait, DbErr, Set, Statement};
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
        self.modified = Set(Local::now().naive_local());
        Ok(self)
    }
}

impl Entity {
    pub async fn incr_fav_count_by_id<C>(db: &C, template_id: i64) -> Result<u64, DbErr>
    where
        C: ConnectionTrait,
    {
        let result = db
            .execute_raw(Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                "update task_template set fav_count=fav_count+1 where id=$1",
                [template_id.into()],
            ))
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn desc_fav_count_by_id<C>(db: &C, template_id: i64) -> Result<u64, DbErr>
    where
        C: ConnectionTrait,
    {
        let result = db
            .execute_raw(Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                "update task_template set fav_count=fav_count-1 where id=$1",
                [template_id.into()],
            ))
            .await?;
        Ok(result.rows_affected())
    }
}
