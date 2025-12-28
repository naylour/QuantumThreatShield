use axum::Router;

pub fn router() -> Router<super::core::state::AppState> {
    Router::new()
}
