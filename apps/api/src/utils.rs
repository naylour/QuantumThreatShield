use config::load_config;

pub fn take_config() -> Result<config::schema::Config, config::schema::ConfigError> {
    let current = std::env::current_dir().expect("Cannot get current dir");

    let env_path = current
        .join("../../infra/env/.env")
        .canonicalize()
        .unwrap_or_else(|_| current.join("../../infra/env/.env"));

    load_config(&env_path)
}
