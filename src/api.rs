use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};

type ConnPool = Pool<Sqlite>;

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlMap {
    pub short_url: String,
    pub long_url: String,
}

#[post("/json")]
async fn json(payload: web::Json<UrlMap>) -> impl Responder {
    tracing::info!("{:?}", payload);
    HttpResponse::Ok().json(UrlMap {
        short_url: payload.short_url.clone(),
        long_url: payload.long_url.clone(),
    })
}

#[get("/short")]
pub async fn short() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .insert_header(("Location", "https://www.google.com"))
        .body("")
}
#[get("/getall")]
async fn get_all(data: Data<ConnPool>) -> impl Responder {
    match sqlx::query_as!(UrlMap, "SELECT short_url, long_url FROM url_map")
        .fetch_all(&data.as_ref().clone())
        .await
    {
        Ok(_urls) => HttpResponse::Ok().body(format!("{_urls:?}")),
        Err(e) => {
            tracing::error!("Error occured while fetching data, {e}");
            HttpResponse::BadRequest().body(format!("{e:?}"))
        }
    }
}

#[get("/insert")]
async fn insert(data: Data<ConnPool>) -> impl Responder {
    match sqlx::query_as!(
        UrlMap,
        r#"
        INSERT INTO url_map (short_url, long_url)
        VALUES ($1, $2)
        RETURNING short_url as "short_url!", long_url as "long_url!"
        "#,
        "foo",
        "www.google.com",
    )
    .fetch_one(&data.as_ref().clone())
    .await
    {
        Err(e) => {
            tracing::error!("Unable to insert the row, {e}");
            HttpResponse::BadRequest().body("")
        }
        Ok(_user) => HttpResponse::Ok().body(""),
    }
}
