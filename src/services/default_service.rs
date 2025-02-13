use axum::http::StatusCode;
use axum::Json;
use crate::db::app_state::AppState;
use crate::models::default_model::{CreateUser, User};
use crate::repositories::default_repository::{get_user_by_id, create_user};

pub async fn get_user(app_state: &AppState, user_id: i32) -> Result<(StatusCode, Json<User>), StatusCode> {
    get_user_by_id(app_state, user_id).await
}

pub async fn add_user(app_state: &AppState, payload: CreateUser) -> Result<(StatusCode, Json<User>), StatusCode> {
    create_user(app_state, payload).await
}