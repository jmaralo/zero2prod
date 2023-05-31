use std::{env, fmt::Display};

use config::{Config, ConfigError, Environment, File};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let base_path = env::current_dir()
        .expect("Failed to get CWD")
        .join("configuration");
    let environment = env::var("APP_ENVIRONMENT").unwrap_or(String::from("dev"));

    Config::builder()
        .add_source(File::from(base_path.join("base")).required(true))
        .add_source(File::from(base_path.join(environment)).required(true))
        .add_source(Environment::with_prefix("app").separator("__"))
        .build()?
        .try_deserialize()
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettigns,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApplicationSettigns {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub database_name: String,
    pub host: String,
}

impl DatabaseSettings {
    pub fn server_connection_string(&self) -> Secret<String> {
        Secret::new(format!("{}", self))
    }

    pub fn database_connection_string(&self) -> Secret<String> {
        Secret::new(format!("{}/{}", self, self.database_name))
    }
}

impl Display for DatabaseSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        )
    }
}
