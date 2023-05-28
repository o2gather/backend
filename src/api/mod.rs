use actix_web::web;
use actix_session::Session;
use actix_web::{post, HttpResponse, Responder};

mod index;
mod identify;
mod types;
mod user_info;

mod identify_test;

use crate::api::index::demo;
use crate::api::types::DefaultError;
use crate::api::identify::{user_login, user_logout};
use crate::api::user_info::{get_user, patch_user};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(demo);
    cfg.service(login_mock);
    cfg.service(web::scope("/api/v1")
        .service(user_login)
        .service(user_logout)
        .service(get_user)
        .service(patch_user)
    );
    // cfg.service(get_all_post);
    // cfg.service(create_post);
    // cfg.service(health_check);
}

#[post("/")]
pub async fn login_mock(
    session: Session,
) -> impl Responder {
    match session.insert("user_id", "test_user_id") {
        Ok(_) => (),
        Err(_) => {
            return HttpResponse::InternalServerError().json(DefaultError {
                message: "Failed to set session".to_string(),
                error_code: "500".to_string(),
            })
        }
    }

    HttpResponse::Ok().finish()
}