use actix_files::Files;
use actix_web::{web::Data, App, HttpServer};
use color_eyre::Report;
mod api;
mod db;
mod logging;

#[tokio::main]
async fn main() -> Result<(), Report> {
    dotenvy::dotenv().ok(); // Load .env file if exist
    logging::log()?;
    let pool = db::setup_db().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(api::minify)
            .service(api::get_all)
            .service(api::short)
            .service(api::insert)
            .service(api::json)
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
