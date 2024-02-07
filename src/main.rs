mod models;
mod routes;
mod state;

use std::env;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router
};
use tracing::info;


async fn root() -> StatusCode {
    StatusCode::OK
}


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    let app = Router::new()
        .nest(
            "/",
            Router::new()
                .route("/", get(root))
                .nest(
                    "/api",
                    Router::new()
                        .nest(
                            "/v1",
                            Router::new()
                        )
                ) 
        );
    
    let app_host = env::var("APP_HOST").unwrap_or("0.0.0.0".to_string());
    let app_port = env::var("APP_PORT").unwrap_or("3000".to_string());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Listening on: {}:{}", app_host, app_port);

    axum::serve(listener, app).await.unwrap();
}
