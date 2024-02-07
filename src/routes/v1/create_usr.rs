use axum::{http::StatusCode, Json};

use crate::models::CreateUser;


async fn create_user(Json(payload): Json<CreateUser>) -> StatusCode {
    

    StatusCode::CREATED
}