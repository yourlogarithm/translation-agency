use sqlx::postgres::PgPool;
use tracing::info;
use std::{env, sync::Arc};


pub struct Environment {
    pub prod: bool,
    pub jwt_secret: Vec<u8>,
}

impl Environment {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();
        let jwt_secret = String::new().into_bytes();
        let prod = match env::var("APP_ENV") {
            Ok(env) => match env.as_str() {
                "development" => false,
                "production" => true,
                _ => return Err("Invalid environment".into()),
            },
            Err(_) => false,
        };
        Ok(Self {
            prod,
            jwt_secret,
        })
    }
}

pub struct AppState {
    pub env: Environment,
    pub pg_pool: PgPool,
}

impl AppState {
    pub async fn init() -> Result<Arc<Self>, Box<dyn std::error::Error>> {
        let database_url = match env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => return Err("DATABASE_URL is not set".into()),
        };
        let pg_pool = PgPool::connect(&database_url).await?;
        let env = Environment::init()?;
        tracing_subscriber::fmt()
            .with_max_level(if env.prod {
                tracing::Level::INFO
            } else {
                tracing::Level::DEBUG
            })
            .init();
        if env.prod {
            info!("Running in production mode");
        } else {
            info!("Running in development mode");
        }
        Ok(Arc::new(Self { env, pg_pool }))
    }
}
