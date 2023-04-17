use actix_web::{get, Result};
use actix_files::NamedFile;
use std::path::PathBuf;


#[get("/")]
pub async fn demo() -> Result<NamedFile> {
    let path = PathBuf::from("static/index.html");
    Ok(NamedFile::open(path)?)
}