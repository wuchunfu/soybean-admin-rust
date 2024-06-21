use serde::Deserialize;

use super::{DatabaseConfig, JwtConfig, ServerConfig};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
}
