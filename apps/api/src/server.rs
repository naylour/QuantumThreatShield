use anyhow::{Context, Result};
use axum::{Router, serve};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::{app::AppState, routes::router};

pub async fn run_server(database_pool: PgPool, port: u16) -> Result<()> {
    logger::info!("Инициализация API...");

    let app_state = AppState {
        database: database_pool,
    };

    let app = Router::new().merge(router()).with_state(app_state);

    let addr = format!("0.0.0.0:{port}");

    logger::info!("Запуск слушателя...");
    let listener = TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Не удалось запустить слушатель по адресу: {addr}"))?;

    serve(listener, app).await?;

    Ok(())
}
