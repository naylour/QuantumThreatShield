use config::{load_config, schema::Config};

use anyhow::{Context, Result};

pub fn take_config() -> Result<Config> {
    let current = std::env::current_dir().with_context(|| "Cannot get current dir".to_string())?;

    let env_path = current
        .join("../infra/env/.env")
        .canonicalize()
        .with_context(|| format!("Не удалось найти файл окружения"))?;

    load_config(&env_path)
}
