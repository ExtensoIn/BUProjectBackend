pub mod app_state;

use sqlx::{PgPool};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

pub async fn get_connection_pool(database_url: String) -> PgPool {
    let opts: PgConnectOptions = database_url.parse().expect("Invalid database URL");

    // opts = opts.log_statements(log::LevelFilter::Trace);
    
    PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect_with(opts)
        .await
        .expect("Failed to create connection pool")
}