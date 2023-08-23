use rbatis::{crud, impl_select, impl_update, sql, RBatis};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
enum StoreType {
    /**
     * 文档数据库，存json格式
     */
    DOC,

    /**
     * 关系数据库，存schema固定的关系表
     */
    RDB,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataStoreMeta {
    id: i64,
    user_id: i64,
    name: String,
    store_type: StoreType,
    created: String,
    modified: String,
}

crud!(DataStoreMeta {});
