use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateUser {
    pub(crate) username: String,
}

#[derive(Serialize, Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
}
