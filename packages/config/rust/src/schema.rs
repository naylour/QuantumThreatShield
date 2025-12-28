use crate::mode::Mode;
use crate::raw_schema;
use serde::{Deserialize, Serialize};
use std::fmt;
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseConfig {
    pub name: String,

    pub user: String,

    pub host: String,

    pub port: u16,

    pub password: Uuid,

    pub url: Url,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RedisConfig {
    pub user: String,

    pub host: String,

    pub port: u16,

    pub password: Uuid,

    pub database: u8,

    pub url: Url,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub port_api: u16,

    pub port_app: u16,

    pub port_agent: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub mode: Mode,

    pub redis: RedisConfig,

    pub database: DatabaseConfig,

    pub app: AppConfig,
}

#[derive(Debug)]
pub enum ConfigError {
    InvalidUrl(String),
    InvalidPort(String),
    InvalidPassword(String),
}

impl TryFrom<raw_schema::Config> for Config {
    type Error = ConfigError;

    fn try_from(value: raw_schema::Config) -> Result<Self, Self::Error> {
        let database_url = Url::parse(&value.database.url)
            .map_err(|_| ConfigError::InvalidUrl("DATABASE_URL".to_string()))?;

        let redis_url = Url::parse(&value.redis.url)
            .map_err(|_| ConfigError::InvalidUrl("REDIS_URL".to_string()))?;

        let database_password = Uuid::parse_str(&value.database.password)
            .map_err(|_| ConfigError::InvalidPassword("DATABASE_PASSWORD".to_string()))?;

        let redis_password = Uuid::parse_str(&value.redis.password)
            .map_err(|_| ConfigError::InvalidPassword("REDIS_PASSWORD".to_string()))?;

        Ok(Config {
            mode: value.mode,

            app: AppConfig {
                port_api: value.app.port_api,
                port_app: value.app.port_app,
                port_agent: value.app.port_agent,
            },
            database: DatabaseConfig {
                name: value.database.name,
                user: value.database.user,
                host: value.database.host,
                port: value.database.port,
                password: database_password,
                url: database_url,
            },
            redis: RedisConfig {
                user: value.redis.user,
                host: value.redis.host,
                port: value.redis.port,
                password: redis_password,
                database: value.redis.database,
                url: redis_url,
            },
        })
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(self).map_err(|_| fmt::Error)?;
        write!(f, "Config {}", pretty_json)
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidUrl(s) => write!(f, "Неверный URL: {}", s),
            ConfigError::InvalidPort(s) => write!(f, "Неверный порт: {}", s),
            ConfigError::InvalidPassword(s) => write!(f, "Неверный пароль: {}", s),
        }
    }
}

impl std::error::Error for ConfigError {}
