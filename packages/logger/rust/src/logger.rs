use anyhow::Result;
use config::mode::Mode;
use std::{fs, path::PathBuf};
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub struct LoggerGuard {
    _guard: tracing_appender::non_blocking::WorkerGuard,
}

pub fn init_logger(mode: &Mode, path: &str) -> Result<LoggerGuard> {
    let logs_dir = PathBuf::from(path);
    fs::create_dir_all(&logs_dir)?;

    // Максимальный уровень логирования
    let log_level = match mode {
        Mode::Dev => Level::DEBUG,
        Mode::Prod => Level::INFO,
    };

    // Rolling файл
    let file_appender = rolling::daily(&logs_dir, "app.log");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    // Фильтр по уровню
    let filter = EnvFilter::from_default_env().add_directive(log_level.into());

    // Слой для файла
    let file_layer = fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_timer(fmt::time::UtcTime::rfc_3339())
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_filter(filter.clone());

    // Слой для консоли
    let console_layer = fmt::layer()
        .with_ansi(true)
        .with_timer(fmt::time::UtcTime::rfc_3339())
        .with_filter(filter);

    // Регистрируем оба слоя
    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();

    Ok(LoggerGuard { _guard: guard })
}
