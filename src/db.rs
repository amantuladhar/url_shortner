use color_eyre::Report;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub async fn setup_db() -> Result<Pool<Sqlite>, Report> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env should be set");
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}
