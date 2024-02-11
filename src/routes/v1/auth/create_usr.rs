use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use regex::Regex;
use tracing::{debug, error, info};

use crate::{
    models::{ApiResponse, CreateUser},
    state::AppState,
};

fn is_username_valid(username: &str) -> bool {
    let re_start = Regex::new(r"^[a-z]").unwrap();
    let re_valid_chars = Regex::new(r"^[a-z0-9_.-]+$").unwrap();
    let re_consecutive_special_chars = Regex::new(r"[_.-]{2,}").unwrap();
    let re_ends_with_special_chars = Regex::new(r"[_.-]$").unwrap();

    re_start.is_match(username)
        && re_valid_chars.is_match(username)
        && !re_consecutive_special_chars.is_match(username)
        && !re_ends_with_special_chars.is_match(username)
}


fn is_name_valid(name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-ZàáâäãåąčćęèéêëėįìíîïłńòóôöõøùúûüųūÿýżźñçčšžÀÁÂÄÃÅĄĆČĖĘÈÉÊËÌÍÎÏĮŁŃÒÓÔÖÕØÙÚÛÜŲŪŸÝŻŹÑßÇŒÆČŠŽ∂ð ,.'-]+$").unwrap();
    re.is_match(name)
}


fn is_email_valid(email: &str) -> bool {
    let re = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    re.is_match(email)
}


#[axum::debug_handler]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<ApiResponse<i32>>) {
    info!("Creating new user with username: {}", payload.usr);

    debug!("Validating username");
    if payload.usr.is_empty() || payload.usr.len() < 6 || payload.usr.len() > 30 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::failed(
                "Username must be between 6 and 30 characters".to_owned(),
            )),
        );
    }

    if !is_username_valid(&payload.usr) {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::failed("Username must start with a letter and contain only letters, numbers, and the characters: ._-".to_owned())));
    }

    debug!("Validating password");
    if payload.pwd.is_empty() || payload.pwd.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::failed("Password must be at least 8 characters".to_owned())),
        );
    }

    if payload.pwd != payload.cpwd {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::failed("Passwords do not match".to_owned())));
    }

    debug!("Validating first and last name");
    if let Some(ref fname) = payload.fname {
        if fname.len() < 2 || fname.len() > 255 {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::failed("First name must be between 2 and 255 characters".to_owned())),
            );
        }
        if !is_name_valid(&fname) {
            return (StatusCode::BAD_REQUEST, Json(ApiResponse::failed("First name contains invalid characters".to_owned())));
        }
    }

    if let Some(ref lname) = payload.lname {
        if lname.len() < 2 || lname.len() > 255 {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::failed("Last name must be between 2 and 255 characters".to_owned())),
            );
        }
        if !is_name_valid(&lname) {
            return (StatusCode::BAD_REQUEST, Json(ApiResponse::failed("Last name contains invalid characters".to_owned())));
        }
    }

    debug!("Validating email");
    if payload.email.len() < 6 || payload.email.len() > 320 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::failed("Email must be between 6 and 320 characters".to_owned())),
        );
    }

    if !is_email_valid(&payload.email) {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::failed("Invalid email".to_owned())));
    }

    debug!("Hashing password");
    let hashed_password = match bcrypt::hash(payload.pwd, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => {
            error!("Failed to hash password: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::failed(format!("Failed to hash password: {}", e))));
        }
    };

    debug!("Inserting user into database");
    let result = match state
        .pg_client
        .query_one(
            "INSERT INTO clients (usr, pwd, fname, lname, email) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            &[&payload.usr, &hashed_password, &payload.fname, &payload.lname, &payload.email],
        )
        .await
    {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to insert user into database: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::failed(format!("Failed to insert user into database: {}", e))));
        }
    };

    // TODO: Send verification email

    (StatusCode::CREATED, Json(ApiResponse::successful(result.get(0))))
}
 