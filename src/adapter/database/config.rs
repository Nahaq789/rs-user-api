use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn establish_pool() -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?)
}
