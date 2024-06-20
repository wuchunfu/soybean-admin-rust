use serde::Deserialize;

use super::{DatabaseConfig, ServerConfig};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}