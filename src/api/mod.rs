use actix_web::web;

mod index;
mod identify;

use crate::api::index::demo;
use crate::api::identify::user_login;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(demo);
    cfg.service(web::scope("/api/v1").service(user_login));
    // cfg.service(get_all_post);
    // cfg.service(create_post);
    // cfg.service(health_check);
}
