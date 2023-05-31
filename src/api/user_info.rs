use actix_session::Session;
use actix_web::{get, patch, web, HttpResponse, Responder};
use uuid::Uuid;

use crate::api::types::{DefaultError};
use crate::MyData;
use crate::PgPooledConnection;
use crate::db::{get_user_by_id, update_user};
use crate::models::User;
use crate::models::UpdateUser;

#[get("/users/{user_id}")]
pub async fn get_user(
    path: web::Path<(Uuid,)>,
    data: web::Data<MyData>,
    session: Session,
) -> impl Responder {
    let user_id: Uuid;
    match session.get::<Uuid>("user_id") {
        Ok(id) => {
            match id {
                Some(id) => {
                    user_id = id;
                }
                None => {
                    return HttpResponse::Forbidden().json(DefaultError {
                        message: "Forbidden".to_string(),
                        error_code: "403".to_string(),
                    });
                }
            }
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(DefaultError {
                message: "Failed to get session".to_string(),
                error_code: "500".to_string(),
            })
        }
    }
    if path.0 != user_id {
        return HttpResponse::Forbidden().json(DefaultError {
            message: "Forbidden".to_string(),
            error_code: "403".to_string(),
        });
    }

    let mut conn: PgPooledConnection = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let user = get_user_by_id(&mut conn, user_id);

    HttpResponse::Ok().json(user)
}

#[patch("/users/{user_id}")]
pub async fn patch_user(
    path: web::Path<(Uuid,)>,
    form: web::Json<UpdateUser>,
    data: web::Data<MyData>,
    session: Session,
) -> impl Responder {
    let user_id: Uuid;
    match session.get::<Uuid>("user_id") {
        Ok(id) => {
            match id {
                Some(id) => {
                    user_id = id;
                }
                None => {
                    return HttpResponse::Forbidden().json(DefaultError {
                        message: "Forbidden".to_string(),
                        error_code: "403".to_string(),
                    });
                }
            }
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(DefaultError {
                message: "Failed to get session".to_string(),
                error_code: "500".to_string(),
            })
        }
    }
    if path.0 != user_id {
        return HttpResponse::Forbidden().json(DefaultError {
            message: "Forbidden".to_string(),
            error_code: "403".to_string(),
        });
    }

    let mut conn: PgPooledConnection = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let have_changes = 
        form.avatar.is_some() || form.name.is_some() ||
        form.email.is_some() || form.phone.is_some();
    let user: User;
    if have_changes {
        user = update_user(&mut conn, user_id, form.into_inner());
    } else {
        user = get_user_by_id(&mut conn, user_id);
    }

    HttpResponse::Ok().json(user)
}