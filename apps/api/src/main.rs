mod app;
mod utils;

use logger::{debug, info, init_logger};
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = utils::take_config().expect("Не удалось загрузить конфигурацию");

    let _logger = init_logger(&config.mode, "../../logs").unwrap();

    info!("Логгирование запущено");
    info!("Инициализация приложения...");
    info!("Конфигурация загружена");
    debug!("\n{}", format!("{}", config));

    Ok(())
}
