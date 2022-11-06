use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use color_eyre::Report;
use sqlx::sqlite::SqlitePoolOptions;
mod setup;

#[derive(Debug)]
struct Person {
    name: Option<String>,
    age: Option<i64>,
}
#[tokio::main]
async fn main() -> Result<(), Report> {
    setup::setup_log()?;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:./test_123123123.db?mode=rwc")
        .await?;
    sqlx::migrate!().run(&pool).await?;

    let new_user = sqlx::query_as!(
        Person,
        r#"
        INSERT INTO person (name, age)
        VALUES ($1, $2)
        RETURNING name, age
        "#,
        "Aman",
        20,
    )
    .fetch_one(&pool.clone())
    .await?;
    println!("INSERT {new_user:?}");

    let person = sqlx::query_as!(Person, "SELECT name, age FROM person")
        .fetch_all(&pool)
        .await?;
    println!("SELECT {person:?}");

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
