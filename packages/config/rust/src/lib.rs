use envconfig::Envconfig;

use crate::schema::Config;

pub mod mode;
pub mod raw_schema;
pub mod schema;

use dotenvy::from_path;
use std::path::PathBuf;

pub fn load_config(path: &PathBuf) -> Result<schema::Config, schema::ConfigError> {
    from_path(&path).ok();

    let raw_config = raw_schema::Config::init_from_env().expect("Failed to read env");

    Config::try_from(raw_config)
}
