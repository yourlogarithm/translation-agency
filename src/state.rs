use sqlx::postgres::PgPool;
use tracing::info;
use std::{env, sync::Arc};


pub struct Environment {
    pub prod: bool,
    pub jwt_secret: &'static [u8],
}

impl Environment {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET")?;
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
            jwt_secret: jwt_secret.as_bytes(),
        })
    }
}

pub struct AppState {
    pub env: Environment,
    pub pg_pool: PgPool,
}

impl AppState {
    pub async fn init() -> Result<Arc<Self>, Box<dyn std::error::Error>> {
        let pg_pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
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
