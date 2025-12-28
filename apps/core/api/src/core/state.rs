use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: PgPool,
}
impl AppState {
    pub fn new(database_pool: PgPool) -> Self {
        Self {
            database: database_pool,
        }
    }
}
