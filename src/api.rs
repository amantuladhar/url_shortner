use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

type ConnPool = Pool<Sqlite>;

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlMap {
    pub short_url: String,
    pub long_url: String,
}

#[derive(Debug, Deserialize)]
pub struct MinifyUrlInput {
    pub url: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(redirect_by_short_url);
    cfg.service(web::scope("/api").configure(|scoped_cfg| {
        scoped_cfg.service(get_all);
        scoped_cfg.service(minify);
    }));
}

#[post("/minify")]
async fn minify(input: web::Json<MinifyUrlInput>, data: Data<ConnPool>) -> impl Responder {
    match sqlx::query_as!(
        UrlMap,
        r#"
            SELECT short_url, long_url
            FROM url_map
            WHERE long_url = ?;
        "#,
        input.url
    )
    .fetch_optional(&data.as_ref().clone())
    .await
    {
        Ok(url) => {
            if let Some(url) = url {
                return HttpResponse::Ok().json(url);
            }
            let url = insert_minified_url(&input.url, &data.as_ref().clone())
                .await
                .unwrap();
            HttpResponse::Ok().json(url)
        }
        Err(e) => {
            tracing::error!("Error {e:?}");
            HttpResponse::BadRequest().json(format!("{e:?}"))
        }
    }
}

#[get("/{short_url}")] // <- define path parameters
async fn redirect_by_short_url(path: web::Path<String>, data: Data<ConnPool>) -> impl Responder {
    tracing::info!("Path :: {path:?}");
    match find_by_short_url(path.as_ref(), &data).await {
        None => HttpResponse::BadRequest().body(""),
        Some(url) => {
            let full_url = format!("https://{url}");
            HttpResponse::PermanentRedirect()
                .insert_header(("Location", full_url))
                .body("")
        }
    }
}

#[get("/get-all")]
async fn get_all(data: Data<ConnPool>) -> impl Responder {
    match sqlx::query_as!(UrlMap, "SELECT short_url, long_url FROM url_map;")
        .fetch_all(&data.as_ref().clone())
        .await
    {
        Ok(urls) => HttpResponse::Ok().json(urls),
        Err(e) => {
            tracing::error!("Error occured while fetching data, {e}");
            HttpResponse::BadRequest().body(format!("{e:?}"))
        }
    }
}

async fn insert_minified_url(url: &str, connection: &Pool<Sqlite>) -> Result<UrlMap, sqlx::Error> {
    let uuid = generate_unique_short_url(connection).await?;
    sqlx::query_as!(
        UrlMap,
        r#"
        INSERT INTO url_map (short_url, long_url)
        VALUES ($1, $2)
        RETURNING short_url as "short_url!", long_url as "long_url!"
        "#,
        uuid,
        url,
    )
    .fetch_one(connection)
    .await
}

async fn generate_unique_short_url(connection: &Pool<Sqlite>) -> Result<String, sqlx::Error> {
    loop {
        let uuid = Uuid::new_v4().to_string();
        let (id, _) = uuid.split_at(8);
        match find_by_short_url(id, connection).await {
            Some(_) => continue,
            None => {
                return Ok(String::from(id));
            }
        }
    }
}

async fn find_by_short_url(short_url: &str, connection: &ConnPool) -> Option<String> {
    match sqlx::query!(
        r#"
        SELECT long_url as "long_url!"
        FROM url_map
        WHERE short_url = ?;
        "#,
        short_url
    )
    .fetch_optional(connection)
    .await
    .unwrap_or(None)
    {
        Some(val) => Some(val.long_url),
        None => None,
    }
}
