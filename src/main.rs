mod api;
mod db;
mod models;
mod schema;

use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::cookie::Key;
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
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let google_client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let secret_key = Key::from(secret_key.as_bytes());
    let pool: PgPool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("Failed to create pool.");

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let session_store = RedisSessionStore::new(redis_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(MyData {
                pool: pool.clone(),
                google_client_id: google_client_id.clone(),
            }))
            .wrap(
                SessionMiddleware::builder(
                    session_store.clone(),
                    secret_key.clone()
                )
                .cookie_name("session".to_string())
                .build()
            )
            .wrap(Logger::default())
            .configure(api::init)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
