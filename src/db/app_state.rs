use sqlx::PgPool;

#[derive(Clone)]
pub(crate) struct AppState{
    pub(crate) pool: PgPool
}

impl AppState {
    pub fn new(pool: PgPool) -> AppState{
        Self{
            pool
        }
    }
}
