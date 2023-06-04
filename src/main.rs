mod api;
mod db;
mod models;
mod schema;

use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenvy::dotenv;
use env_logger::Env;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct MyData {
    pool: PgPool,
    google_client_id: String,
    redirect_url: String,
    cors_enabled: bool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let google_client_id: String = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let redis_url: String = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let secret_key: String = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let stage: String = env::var("STAGE").unwrap_or("dev".to_string());
    let secret_key: Key = Key::from(secret_key.as_bytes());
    let redirect_url: String = env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");
    let cors_enabled: bool = env::var("CORS").unwrap_or("false".to_string()) == "true";
    let pool: PgPool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("Failed to create pool.");

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let session_store: RedisSessionStore = RedisSessionStore::new(redis_url).await.unwrap();
    let addr: &str = if stage == "dev" {
        "127.0.0.1"
    } else {
        "0.0.0.0"
    };

    HttpServer::new(move || {
        let cors: Cors;
        let same_site: actix_web::cookie::SameSite;
        if cors_enabled {
            cors = Cors::default()
                .supports_credentials()
                .allow_any_header()
                .allow_any_method()
                .allowed_origin_fn(
                    |_origin: &actix_web::http::header::HeaderValue,
                     _req_head: &actix_web::dev::RequestHead| true,
                )
                .max_age(3600);
            same_site = actix_web::cookie::SameSite::None;
        } else {
            cors = Cors::default();
            same_site = actix_web::cookie::SameSite::Lax;
        }
        let logger: Logger =
            Logger::new("%{r}a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")
                .exclude("/ping");

        App::new()
            .app_data(web::Data::new(MyData {
                pool: pool.clone(),
                google_client_id: google_client_id.clone(),
                redirect_url: redirect_url.clone(),
                cors_enabled: cors_enabled,
            }))
            .wrap(
                SessionMiddleware::builder(session_store.clone(), secret_key.clone())
                    .cookie_name("session".to_string())
                    .cookie_same_site(same_site)
                    .build(),
            )
            .wrap(logger)
            .wrap(cors)
            .configure(api::init)
    })
    .bind((addr, 8080))?
    .run()
    .await
}
