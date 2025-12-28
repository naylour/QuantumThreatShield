use super::api::Api;
use anyhow::Context;
use logger;

impl Api {
    pub async fn run(&self, port: u16) -> anyhow::Result<()> {
        logger::info!("Инициализация API...");

        let app = axum::Router::new()
            .merge(crate::routes::router())
            .with_state(self.state.clone());

        let addr = format!("0.0.0.0:{port}");

        logger::info!("Запуск слушателя...");

        let tcp_listener = tokio::net::TcpListener::bind(&addr).await;

        let listener = tcp_listener
            .with_context(|| format!("Не удалось запустить слушатель по адресу: {addr}"))?;

        axum::serve(listener, app).await?;

        Ok(())
    }
}
