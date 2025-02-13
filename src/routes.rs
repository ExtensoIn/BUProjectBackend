mod default_routes;

use crate::db::app_state::AppState;
use crate::routes::default_routes::get_default_routes;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::Router;

pub fn get_routes(app_state: AppState) -> Router {
    let frontend_url = std::env::var("FRONTEND_URL").expect("Frontend url must be set");
    Router::new()
        .merge(get_default_routes())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
                .allow_headers([CONTENT_TYPE, AUTHORIZATION])
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]),
        )
        .with_state(app_state)
}
