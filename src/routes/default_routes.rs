use axum::Router;
use axum::routing::{get, post};
use crate::controllers::default_controller::{create_user, get_user};
use crate::db::app_state::AppState;

pub(crate) fn get_default_routes() -> Router<AppState> {
    Router::new()
        .route("/default/{user_id}", get(get_user))
        .route("/default", post(create_user))
}