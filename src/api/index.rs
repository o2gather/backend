use actix_web::{get, Result, HttpResponse, Responder};
use actix_files::NamedFile;
use std::path::PathBuf;
use crate::api::types::{DefaultMsg};

#[get("/ping")]
async fn ping() -> impl Responder  {
    HttpResponse::Ok().json(DefaultMsg {
        message: "pong".to_string(),
        message_code: "200".to_string()
    })
}

#[get("/")]
pub async fn demo() -> Result<NamedFile> {
    let path = PathBuf::from("static/index.html");
    Ok(NamedFile::open(path)?)
}