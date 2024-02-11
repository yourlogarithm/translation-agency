use std::env;
use sqlx::postgres::PgPool;

pub struct Env {
    pub prod: bool,
    pub jwt_secret: String,
}

impl Env {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            prod: env::var("PROD").unwrap_or_default().parse()?,
            jwt_secret: env::var("JWT_SECRET")?
        })
    }
}


pub struct AppState {
    pub env: Env,
    pub pg_pool: PgPool
}
