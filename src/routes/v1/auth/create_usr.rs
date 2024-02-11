use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use regex::Regex;
use tracing::{debug, error, info};

use crate::{
    email::VerifyEmailTemplate, models::{ApiResponse, CreateUser}, state::AppState
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
            Json(ApiResponse::failed_str(
                "Username must be between 6 and 30 characters",
            )),
        );
    }

    if !is_username_valid(&payload.usr) {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::failed_str("Username must start with a letter and contain only letters, numbers, and the characters: ._-")));
    }

    debug!("Validating password");
    if payload.pwd.is_empty() || payload.pwd.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::failed_str("Password must be at least 8 characters")),
        );
    }

    if payload.pwd != payload.cpwd {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::failed_str("Passwords do not match")));
    }

    debug!("Validating first and last name");
    if let Some(ref fname) = payload.fname {
        if fname.len() < 2 || fname.len() > 255 {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::failed_str("First name must be between 2 and 255 characters")),
            );
        }
        if !is_name_valid(&fname) {
            return (StatusCode::BAD_REQUEST, Json(ApiResponse::failed_str("First name contains invalid characters")));
        }
    }

    if let Some(ref lname) = payload.lname {
        if lname.len() < 2 || lname.len() > 255 {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::failed_str("Last name must be between 2 and 255 characters")),
            );
        }
        if !is_name_valid(&lname) {
            return (StatusCode::BAD_REQUEST, Json(ApiResponse::failed_str("Last name contains invalid characters")));
        }
    }

    debug!("Validating email");
    if payload.email.len() < 6 || payload.email.len() > 320 {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::failed_str("Email must be between 6 and 320 characters")),
        );
    }

    if !is_email_valid(&payload.email) {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::failed_str("Invalid email")));
    }

    if match state
        .pg_pool
        .query_one("SELECT EXISTS (SELECT 1 FROM clients WHERE email = $1)", &[&payload.email])
        .await {
        Ok(r) => r.get(0),
        Err(e) => {
            let msg = format!("Failed to check if email exists: {}", e);
            error!(msg);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::failed_string(msg)));
        }
    } {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::failed_str("Email already exists")));
    }

    if match state
        .pg_pool
        .query_one("SELECT EXISTS (SELECT 1 FROM clients WHERE usr = $1)", &[&payload.usr])
        .await {
        Ok(r) => r.get(0),
        Err(e) => {
            return (StatusCode::BAD_REQUEST, Json(ApiResponse::failed_string(format!("Failed to check if username exists: {}", e))));
        }
    } {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::failed_str("Username already exists")));
    } 

    debug!("Hashing password");
    let hashed_password = match bcrypt::hash(payload.pwd, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => {
            let msg = format!("Failed to hash password: {}", e);
            error!(msg);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::failed_string(msg)));
        }
    };

    let uuid = uuid::Uuid::new_v4().to_string();
    debug!("Inserting user into database - {}", uuid);
    let idx: i32 = match state
        .pg_pool
        .query_one(
            "INSERT INTO clients (usr, pwd, fname, lname, email, vc) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
            &[&payload.usr, &hashed_password, &payload.fname, &payload.lname, &payload.email, &uuid],
        )
        .await
    {
        Ok(r) => r.get(0),
        Err(e) => {
            let msg = format!("Failed to insert user into database: {}", e);
            error!(msg);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::failed_string(msg)));
        }
    };
    debug!("Inserted user into database with id: {}", idx);

    let template = VerifyEmailTemplate {
        app_name: "My App",
        verify_link: &format!("http://localhost:8000/api/v1/verify_email?token={}", uuid),
    };

    (StatusCode::CREATED, Json(ApiResponse::successful(idx)))
}
 
