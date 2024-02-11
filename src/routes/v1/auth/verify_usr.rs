use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use tracing::{error, info};

use crate::{
    models::ApiResponse,
    state::AppState,
};


#[axum::debug_handler]
pub async fn verify_user(
    State(state): State<Arc<AppState>>,
    Query(token): Query<String>,
) -> (StatusCode, Json<ApiResponse<()>>) {
    info!("Verifying token {}", token);
    let username: String = match state
        .pg_pool
        .query("UPDATE clients SET v = true WHERE vc = $1 RETURNING usr", &[&token])
        .await {
        Ok(rows) => {
            if let Some(row) = rows.first() {
                row.get(0)
            } else {
                return (StatusCode::NOT_FOUND, Json(ApiResponse::failed("Token not found")));
            }
        },
        Err(e) => {
            error!("Error querying client: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::failed("Error querying client")));
        }
    };
    info!("User {} verified", username);
    (StatusCode::OK, Json(ApiResponse::successful(())))
}
