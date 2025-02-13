use axum::http::StatusCode;
use axum::Json;
use sqlx::query_as;
use crate::db::app_state::AppState;
use crate::models::default_model::{CreateUser, User};

pub async fn get_user_by_id(app_state: &AppState, user_id: i32) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = query_as::<_, User>("SELECT * FROM users where id = $1")
        .bind(user_id)
        .fetch_one(&app_state.pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(user)))
}

pub async fn create_user(app_state: &AppState, payload: CreateUser) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = query_as::<_, User>(
            r#"insert into users (username) values ($1) returning id, username"#
        )
        .bind(payload.username)
        .fetch_one(&app_state.pool)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok((StatusCode::CREATED, Json(user)))
}