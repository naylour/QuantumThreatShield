use sqlx::PgPool;

pub struct Api {
    pub state: super::state::AppState,
}

impl Api {
    pub fn new(database_pool: PgPool) -> Self {
        Self {
            state: super::state::AppState {
                database: database_pool,
            },
        }
    }
}
