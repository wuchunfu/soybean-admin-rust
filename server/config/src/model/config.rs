use std::collections::HashMap;

use serde::Deserialize;

use super::{DatabaseConfig, JwtConfig, ServerConfig};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    #[allow(dead_code)]
    pub databases: Option<HashMap<String, DatabaseConfig>>,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
}
