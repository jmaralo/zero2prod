use std::fmt::Display;

use config::{Config, ConfigError, File};
use serde::Deserialize;

pub fn get_configuration() -> Result<Settings, ConfigError> {
    Config::builder()
        .add_source(File::with_name("config"))
        .build()?
        .try_deserialize()
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub database_name: String,
    pub host: String,
}

impl DatabaseSettings {
    pub fn server_connection_string(&self) -> String {
        format!("{}", self)
    }

    pub fn database_connection_string(&self) -> String {
        format!("{}/{}", self, self.database_name)
    }
}

impl Display for DatabaseSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
