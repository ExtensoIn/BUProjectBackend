use axum::{
    http::StatusCode,
    Json,
    extract::Path,
};
use axum::extract::State;
use crate::db::app_state::AppState;
use crate::models::default_model::{CreateUser, User};
use crate::services::default_service;

pub async fn get_user(State(app_state): State<AppState>, Path(user_id): Path<i32>) -> Result<Json<User>, StatusCode> {
    let user = default_service::get_user(&app_state, user_id).await?;
    Ok(user.1)
}

pub async fn create_user(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let user = default_service::add_user(&app_state, payload).await?;
    Ok(user.1)
}