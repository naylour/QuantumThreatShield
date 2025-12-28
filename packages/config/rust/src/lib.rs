use envconfig::Envconfig;

use crate::schema::Config;

pub mod mode;
pub mod raw_schema;
pub mod schema;

use anyhow::{Context, Result, anyhow};
use dotenvy;
use std::path::PathBuf;

pub fn load_config(path: &PathBuf) -> Result<Config> {
    // Пытаемся прочитать .env файл, ошибки с контекстом
    dotenvy::from_path(path).with_context(|| {
        format!(
            "Не удалось прочесть конфигурацию из файла: {}",
            path.display()
        )
    })?;

    // Загружаем переменные из окружения
    let raw_config = raw_schema::Config::init_from_env().with_context(|| {
        format!(
            "Не удалось прочесть конфигурацию из окружения или файла: {}",
            path.display()
        )
    })?;

    // Конвертируем в наш тип Config
    let config = Config::try_from(raw_config).map_err(|e| anyhow!(e))?;

    Ok(config)
}
