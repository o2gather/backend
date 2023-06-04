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
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let google_client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let stage = env::var("STAGE").unwrap_or("dev".to_string());
    let secret_key = Key::from(secret_key.as_bytes());
    let redirect_url = env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");
    let cors_enabled = env::var("CORS").unwrap_or("false".to_string()) == "true";
    let pool: PgPool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("Failed to create pool.");

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let session_store = RedisSessionStore::new(redis_url).await.unwrap();
    let addr = if stage == "dev" {
        "127.0.0.1"
    } else {
        "0.0.0.0"
    };

    HttpServer::new(move || {
        let cors: Cors;
        if cors_enabled {
            cors = Cors::permissive()
                .max_age(3600);
        } else {
            cors = Cors::default();
        }
        let logger =
            Logger::new("%{r}a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T").exclude("/ping");

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
