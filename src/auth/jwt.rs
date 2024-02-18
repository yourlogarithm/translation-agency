use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use jsonwebtoken::{DecodingKey, Validation};
use tracing::error;

use crate::{
    models::{ApiResponse, User},
    state::AppState,
};

use super::TokenClaims;

pub async fn auth_middleware<'a>(
    cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiResponse<()>>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        })
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::failed("Token not provided")),
            )
        })?;

    let claims = jsonwebtoken::decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(&state.env.jwt_secret),
        &Validation::default(),
    )
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::failed("Invalid token")),
        )
    })?
    .claims;

    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::failed("Invalid token")),
        )
    })?;

    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(&state.pg_pool)
        .await
        .map_err(|e| {
            let msg = format!("Error fetching response from database: {e}");
            error!(msg);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::failed(msg)),
            )
        })?;

    let user = user.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::failed(
                "The user belonging to this token no longer exists",
            )),
        )
    })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
