mod models;
mod routes;
mod state;
mod enums;

use std::{env, sync::Arc};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use tokio_postgres::NoTls;
use tracing::{info, error};

use crate::{
    routes::v1::auth::create_user, 
    state::AppState
};

async fn root() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let (client, connection) =
        tokio_postgres::connect(&std::env::var("PG_URI").expect("PG_URI not set"), NoTls)
            .await
            .expect("Failed to connect to PostgresSQL");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("Connection error: {}", e);
        }
    });

    let state = Arc::new(AppState {
        pg_client: client
    });

    let app = Router::new().nest(
        "/",
        Router::new().route("/", get(root)).nest(
            "/api",
            Router::new().nest(
                "/v1",
                Router::new().route("/user_creation", post(create_user)),
            ).with_state(state),
        ),
    );

    let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").unwrap_or("8000".to_string());

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", app_host, app_port))
        .await
        .unwrap();

    info!("Listening on: {}:{}", app_host, app_port);

    axum::serve(listener, app).await.unwrap();
}
