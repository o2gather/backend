use actix_session::Session;
use actix_web::{get, put, post, HttpResponse, Responder, web};
use uuid::Uuid;

use crate::api::types::{DefaultError, DefaultMsg};
use crate::MyData;
use crate::models::{NewEventMember, NewEventMsg};
use crate::db;

#[put("/events/{event_id}/join")]
pub async fn join_event(
    path: web::Path<(Uuid,)>,
    mut form: web::Json<NewEventMember>,
    data: web::Data<MyData>,
    session: Session,
) -> impl Responder {
    let user_id: Uuid;
    let event_id = path.0;
    let null_uuid = Uuid::nil();

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

    let mut conn = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let event = db::get_event_by_id(&mut conn, event_id, null_uuid);
    if event.is_none() {
        return HttpResponse::NotFound().json(DefaultError {
            message: "Event not found".to_string(),
            error_code: "404".to_string(),
        });
    }
    let event = event.unwrap();
    if event.user_id == user_id {
        return HttpResponse::BadRequest().json(DefaultError {
            message: "You are the owner of this event".to_string(),
            error_code: "400".to_string(),
        });
    }

    form.event_id = event_id;
    form.user_id = user_id;
    let result = db::create_event_member(&mut conn, form.into_inner());

    match result {
        Ok(_) => (),
        Err(diesel::result::Error::RollbackTransaction) => {
            return HttpResponse::BadRequest().json(DefaultError {
                message: "Have already Reach Max Limit".to_string(),
                error_code: "400".to_string(),
            })
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(DefaultError {
                message: "Failed to join event".to_string(),
                error_code: "500".to_string(),
            })
        }
    }

    HttpResponse::Ok().json(DefaultMsg {
        message: "Success".to_string(),
        message_code: "200".to_string(),
    })
}

#[post("/events/{event_id}/leave")]
pub async fn leave_event(
    path: web::Path<(Uuid,)>,
    data: web::Data<MyData>,
    session: Session,
) -> impl Responder {
    let user_id: Uuid;
    let event_id = path.0;
    let null_uuid = Uuid::nil();

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

    let mut conn = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let event = db::get_event_by_id(&mut conn, event_id, null_uuid);
    if event.is_none() {
        return HttpResponse::NotFound().json(DefaultError {
            message: "Event not found".to_string(),
            error_code: "404".to_string(),
        });
    }
    let event = event.unwrap();
    if event.user_id == user_id {
        return HttpResponse::BadRequest().json(DefaultError {
            message: "You are the owner of this event".to_string(),
            error_code: "400".to_string(),
        });
    }

    let result = db::delete_event_member(&mut conn, event_id, user_id);

    if result == false {
        return HttpResponse::BadRequest().json(DefaultError {
            message: "You are not in this event".to_string(),
            error_code: "400".to_string(),
        });
    }

    HttpResponse::Ok().json(DefaultMsg {
        message: "Success".to_string(),
        message_code: "200".to_string(),
    })
}

#[post("/events/{event_id}/msgs")]
pub async fn add_event_msg(
    path: web::Path<(Uuid,)>,
    mut form: web::Json<NewEventMsg>,
    data: web::Data<MyData>,
    session: Session,
) -> impl Responder {
    let user_id: Uuid;
    let event_id = path.0;

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

    let mut conn = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let event = db::get_event_by_id(&mut conn, event_id, Uuid::nil());
    if event.is_none() {
        return HttpResponse::NotFound().json(DefaultError {
            message: "Event not found".to_string(),
            error_code: "404".to_string(),
        });
    }
    let event = event.unwrap();
    if event.user_id != user_id {
        let event_member = db::get_event_members(&mut conn, event_id);
        if event_member.iter().any(|x| *x == user_id) == false {
            return HttpResponse::Forbidden().json(DefaultError {
                message: "You are not in this event".to_string(),
                error_code: "403".to_string(),
            });
        }
    }

    form.event_id = event_id;
    form.user_id = user_id;
    let result = db::create_event_msg(&mut conn, form.into_inner());

    match result {
        Ok(r) => {
            HttpResponse::Ok().json(r)
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(DefaultError {
                message: "Failed to add event msg".to_string(),
                error_code: "500".to_string(),
            })
        }
    }

   
}

#[get("/events/{event_id}/msgs")]
pub async fn get_event_msgs(
    path: web::Path<(Uuid,)>,
    data: web::Data<MyData>,
    session: Session,
) -> impl Responder {
    let user_id: Uuid;
    let event_id = path.0;

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

    let mut conn = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let event = db::get_event_by_id(&mut conn, event_id, Uuid::nil());
    if event.is_none() {
        return HttpResponse::NotFound().json(DefaultError {
            message: "Event not found".to_string(),
            error_code: "404".to_string(),
        });
    }

    let result = db::get_event_msg_by_event_id(&mut conn, event_id);
    match result {
        Ok(r) => {
            return HttpResponse::Ok().json(r)
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(DefaultError {
                message: "Failed to get event msg".to_string(),
                error_code: "500".to_string(),
            })
        }
    }
}

#[get("/categories")]
pub async fn get_categories(
    data: web::Data<MyData>,
) -> impl Responder {
    let mut conn = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let categories = db::get_categories(&mut conn);

    HttpResponse::Ok().json(categories)
}