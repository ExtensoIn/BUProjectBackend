mod db;
mod routes;
mod services;
mod controllers;
mod models;
mod repositories;

use std::env;
use dotenvy::dotenv;
use tracing::info;
use crate::db::app_state::AppState;
use crate::db::get_connection_pool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::fmt().init();
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    info!("🦀 Starting server...");
    let pool = get_connection_pool(database_url).await;
    let app = routes::get_routes(AppState::new(pool));
    let url = env::var("URL")
        .expect("URL must be set");
    let port = env::var("PORT")
        .expect("PORT must be set");
    let listener = tokio::net::TcpListener::bind(format!("{url}:{port}")).await?;
    info!("🦀 Server running on: {url}:{port}");
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
