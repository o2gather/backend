use actix_web::web;
use actix_session::Session;
use actix_web::{post, HttpResponse, Responder};

mod index;
mod identify;
mod types;
mod user_info;
mod events;
mod event_related;

mod identify_test;
mod index_test;

use crate::api::index::{demo, ping};
use crate::api::types::DefaultError;
use crate::api::identify::{user_login, user_logout};
use crate::api::user_info::{get_user, patch_user};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(demo);
    cfg.service(ping);
    cfg.service(login_mock);
    cfg.service(web::scope("/api/v1")
        .service(user_login)
        .service(user_logout)
        .service(get_user)
        .service(patch_user)
        .service(events::create_event)
        .service(events::get_events)
        .service(events::get_event)
        .service(events::get_user_events)
        .service(events::patch_event)
        .service(events::delete_event)
        .service(event_related::join_event)
        .service(event_related::leave_event)
        .service(event_related::add_event_msg)
        .service(event_related::get_event_msgs)
        .service(event_related::get_categories)
    );
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