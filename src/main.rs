use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use color_eyre::Report;
use sqlx::sqlite::SqlitePoolOptions;
mod setup;

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup::setup_log()?;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:./test_123123123.db?mode=rwc")
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    HttpServer::new(|| App::new().service(hello).service(short))
        .workers(2)
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;
    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/short")]
async fn short() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .insert_header(("Location", "https://www.google.com"))
        .body("")
}
