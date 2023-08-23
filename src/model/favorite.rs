use rbatis::{crud, impl_select, impl_update, sql, RBatis};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Favorite {
    id: i64,
    template_id: i64,
    user_id: i64,
    created: String,
}

crud!(Favorite {});

impl_select!(Favorite{select_by_user_id(user_id:i64)->Option => "`where user_id=#{user_id}`"});
