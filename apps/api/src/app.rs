pub mod json;
pub mod response;
pub mod state;

use sqlx::PgPool;
pub use state::AppState;

use axum::Router;
