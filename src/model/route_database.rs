use ormlite::model::*;

#[derive(Debug, Model)]
#[ormlite(table = "route_database")]
pub struct RouteDatabase {
    #[ormlite(primary_key)]
    pub id: i64,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: i32,
    pub database: String,
    pub disabled: bool,
}
