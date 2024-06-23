use serde::Deserialize;

use super::{DatabaseConfig, JwtConfig, ServerConfig};
use crate::DatabasesConfig;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub databases: Option<Vec<DatabasesConfig>>,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
}
