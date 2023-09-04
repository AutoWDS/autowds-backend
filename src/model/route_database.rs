use ormlitex::model::*;

#[derive(Debug, Model)]
#[ormlitex(table = "route_database")]
pub struct RouteDatabase {
    #[ormlitex(primary_key)]
    pub id: i64,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: i32,
    pub database: String,
    pub disabled: bool,
}
