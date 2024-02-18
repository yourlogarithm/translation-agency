mod auth;
mod email;
mod enums;
mod models;
mod routes;
mod state;

use std::env;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tracing::{error, info};

use crate::{
    routes::v1::user::create_user,
    state::AppState,
};

async fn root() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let state = match AppState::init().await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Couldn't initialize application state: {e}");
            return;
        }
    };

    let app = Router::new().nest(
        "/",
        Router::new().nest(
            "/api",
            Router::new()
                .route("/health", get(root))
                .nest(
                    "/v1",
                    Router::new().nest(
                        "/user",
                        Router::new()
                            .route("/new", post(create_user))
                            .route("/verification", get(create_user)),
                    ),
                )
                .with_state(state),
        ),
    );

    let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").unwrap_or("8000".to_string());
    let listener = match tokio::net::TcpListener::bind(format!("{}:{}", app_host, app_port)).await {
        Ok(listener) => listener,
        Err(e) => {
            error!("Couldn't bind to {}:{}", app_host, app_port);
            error!("{:?}", e);
            return;
        }
    };
    info!("Listening on: {}:{}", app_host, app_port);
    match axum::serve(listener, app).await {
        Ok(_) => (),
        Err(e) => {
            error!("Server error: {:?}", e);
        }
    }
}
