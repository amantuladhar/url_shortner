use actix_web::{web::Data, App, HttpServer};
use color_eyre::Report;
mod api;
mod asset;
mod db;
mod logging;

#[tokio::main]
async fn main() -> Result<(), Report> {
    dotenvy::dotenv().ok(); // Load .env file if exist
    logging::log()?;
    let pool = db::setup_db().await?;

    HttpServer::new(move || {
        App::new()
            .service(asset::index)
            .configure(api::config)
            .app_data(Data::new(pool.clone()))
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
