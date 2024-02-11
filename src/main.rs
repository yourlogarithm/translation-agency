mod email;
mod enums;
mod models;
mod routes;
mod state;
mod auth;

use std::{env, sync::Arc};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tracing::{error, info, Level};

use crate::{routes::v1::auth::create_user, state::{AppState, Env}};

async fn root() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let prod: bool = std::env::var("PROD").unwrap_or_default().parse().unwrap();
    if prod {
        tracing_subscriber::fmt::init();
        info!("Starting in production mode...");
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
        info!("Starting in development mode...");
    }

    let pg_pool = match sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(env!("DATABASE_URL"))
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            error!("Couldn't connect to database {e}");
            return;
        }
    };

    let env = match Env::init() {
        Ok(env) => env,
        Err(e) => {
            error!("Couldn't get environment variables {e}");
            return;
        }
    };

    let state = Arc::new(AppState { 
        env,
        pg_pool 
    });

    let app = Router::new().nest(
        "/",
        Router::new().route("/", get(root)).nest(
            "/api",
            Router::new()
                .nest(
                    "/v1",
                    Router::new().route("/user_creation", post(create_user)),
                )
                .with_state(state),
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
