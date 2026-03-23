use serde::Deserialize;
use summer::config::Configurable;

#[derive(Deserialize, Configurable)]
#[config_prefix = "email"]
pub struct Email {
    pub from: String,
}
