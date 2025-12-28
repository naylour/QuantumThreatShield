mod utils;

use anyhow::Result;
use api;
use database::init_database;
use logger::{self, init_logger};

#[tokio::main]
async fn main() -> Result<()> {
    let config = match utils::take_config() {
        Ok(_config) => {
            println!("Конфигурация загружена. Режим работы: {}", &_config.mode);
            _config
        }
        Err(error) => {
            eprintln!("{:#?}", error);
            std::process::exit(1);
        }
    };

    let _logger = match init_logger(&config.mode, "../logs") {
        Ok(_guard) => {
            logger::info!("Логгирование запущено");
            _guard
        }
        Err(error) => {
            logger::error!("{:#?}", error);
            std::process::exit(1);
        }
    };

    logger::info!("Инициализация приложения...");
    logger::info!("Попытка подключения к базе данных...");
    logger::debug!("\n{}", format!("{}", config));

    let database_pool = match init_database(&config.database.url.to_string()).await {
        Ok(pool) => {
            logger::info!("Подключение к базе успешно");
            pool
        }
        Err(err) => {
            logger::error!("{:#?}", err);
            std::process::exit(1); // Можно завершить программу
        }
    };

    api::Api::new(database_pool)
        .run(config.app.port_api)
        .await?;

    Ok(())
}
