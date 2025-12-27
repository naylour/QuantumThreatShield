use crate::mode::Mode;
use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct DatabaseConfig {
    #[envconfig(from = "DATABASE_NAME", default = "postgres")]
    pub name: String,

    #[envconfig(from = "DATABASE_USER", default = "postgres")]
    pub user: String,

    #[envconfig(from = "DATABASE_HOST", default = "localhost")]
    pub host: String,

    #[envconfig(from = "DATABASE_PORT", default = "5432")]
    pub port: u16,

    #[envconfig(from = "DATABASE_PASSWORD")]
    pub password: String,

    #[envconfig(from = "DATABASE_URL")]
    pub url: String,
}

#[derive(Envconfig, Debug)]
pub struct RedisConfig {
    #[envconfig(from = "REDIS_NAME", default = "QMS_Redis")]
    pub user: String,

    #[envconfig(from = "REDIS_HOST", default = "redis")]
    pub host: String,

    #[envconfig(from = "REDIS_PORT", default = "6379")]
    pub port: u16,

    #[envconfig(from = "REDIS_PASSWORD")]
    pub password: String,

    #[envconfig(from = "REDIS_DATABASE", default = "0")]
    pub database: u8,

    #[envconfig(from = "REDIS_URL")]
    pub url: String,
}

#[derive(Envconfig, Debug)]
pub struct AppConfig {
    #[envconfig(from = "PORT_API")]
    pub port_api: u16,

    #[envconfig(from = "PORT_APP")]
    pub port_app: u16,

    #[envconfig(from = "PORT_AGENT")]
    pub port_agent: u16,
}

#[derive(Envconfig, Debug)]
pub struct Config {
    #[envconfig(from = "MODE", default = "DEV")]
    pub mode: Mode,

    #[envconfig(nested = true)]
    pub redis: RedisConfig,

    #[envconfig(nested = true)]
    pub database: DatabaseConfig,

    #[envconfig(nested = true)]
    pub app: AppConfig,
}
