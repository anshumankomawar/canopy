use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub async fn establish_connection_sqlx() -> PgPool {
    tracing::debug!("connecting to sqlx");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::debug!("connected to sqlx");
    tracing::debug!("shared pool");
    PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("unable to connect to database")
}
