use actix_session::Session;
use actix_web::{get, post, patch, delete, HttpResponse, Responder, web};
use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::api::types::{DefaultError, DefaultMsg};
use crate::MyData;
use crate::models::{NewEvent, UpdateEvent, EventWithMembers};
use crate::PgPooledConnection;
use crate::db;

fn time_check(start_time: NaiveDateTime, end_time: NaiveDateTime) -> bool {
    if start_time > end_time {
        return false;
    }
    return true;
}

fn amount_check(min_amount: i64, max_amount: i64) -> bool {
    if min_amount > max_amount {
        return false;
    }
    return true;
}

#[post("/events")]
pub async fn create_event(
    mut form: web::Json<NewEvent>,
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

    form.user_id = user_id;
    if time_check(form.start_time, form.end_time) == false {
        return HttpResponse::BadRequest().json(DefaultError {
            message: "Start time should be earlier than end time".to_string(),
            error_code: "400".to_string(),
        });
    }
    if amount_check(form.min_amount, form.max_amount) == false {
        return HttpResponse::BadRequest().json(DefaultError {
            message: "Min amount should be smaller than max amount".to_string(),
            error_code: "400".to_string(),
        });
    }
    let mut conn: PgPooledConnection = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let event = db::create_event(&mut conn, form.into_inner());

    HttpResponse::Ok().json(event)
}

#[get("/events")]
pub async fn get_events(
    data: web::Data<MyData>,
) -> impl Responder {
    let mut conn: PgPooledConnection = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let events = db::get_events(&mut conn);

    HttpResponse::Ok().json(events)
}

#[get("/events/{event_id}")]
pub async fn get_event(
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
                    user_id = Uuid::nil();
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
    
    let mut conn: PgPooledConnection = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let event = db::get_event_by_id(&mut conn, path.0, user_id);

    match event {
        Some(event) => {
            return HttpResponse::Ok().json(event);
        }
        None => {
            return HttpResponse::NotFound().json(DefaultError {
                message: "Not Found".to_string(),
                error_code: "404".to_string(),
            });
        }
    }
}

#[patch("/events/{event_id}")]
pub async fn patch_event(
    path: web::Path<(Uuid,)>,
    mut form: web::Json<UpdateEvent>,
    data: web::Data<MyData>,
    session: Session,
) -> impl Responder {
    let user_id: Uuid;
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

    let mut conn: PgPooledConnection = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let data = db::get_event_by_id(&mut conn, path.0, null_uuid);
    let mut event: EventWithMembers;
    match data{
        Some(e) => {
            event = e;
        }
        None => {
            return HttpResponse::NotFound().json(DefaultError {
                message: "Not Found".to_string(),
                error_code: "404".to_string(),
            });
        }
    }

    if event.user_id!= user_id {
        return HttpResponse::Forbidden().json(DefaultError {
            message: "Forbidden".to_string(),
            error_code: "403".to_string(),
        });
    }
    match form.established {
        Some(established) => {
            if established == false {
                return HttpResponse::BadRequest().json(DefaultError {
                    message: "established can only be set to true".to_string(),
                    error_code: "400".to_string(),
                });
            }
        }
        None => {}
    }

    if event.end_time < chrono::Local::now().naive_local() {
        let new_form = UpdateEvent {
            established: form.established,
            start_time: None,
            end_time: None,
            max_amount: None,
            min_amount: None,
            category: None,
            name: None,
            description: None,
        };
        form = web::Json(new_form);
    }

    if form.start_time.is_some() {
        event.start_time = form.start_time.unwrap();
    }
    if form.end_time.is_some() {
        event.end_time = form.end_time.unwrap();
    }
    if form.max_amount.is_some() {
        event.max_amount = form.max_amount.unwrap();
    }
    if form.min_amount.is_some() {
        event.min_amount = form.min_amount.unwrap();
    }

    if time_check(event.start_time, event.end_time) == false {
        return HttpResponse::BadRequest().json(DefaultError {
            message: "Start time should be earlier than end time".to_string(),
            error_code: "400".to_string(),
        });
    }
    if amount_check(event.min_amount, event.max_amount) == false {
        return HttpResponse::BadRequest().json(DefaultError {
            message: "Min amount should be smaller than max amount".to_string(),
            error_code: "400".to_string(),
        });
    }
    
    let have_changes = form.established.is_some()
        || form.start_time.is_some()
        || form.end_time.is_some()
        || form.max_amount.is_some()
        || form.min_amount.is_some()
        || form.category.is_some()
        || form.name.is_some()
        || form.description.is_some();
    
    let event: EventWithMembers;
    if have_changes {
        event = db::update_event(&mut conn, path.0, form.into_inner());
    } else {
        event = db::get_event_by_id(&mut conn, path.0, user_id).unwrap();
    }
    
    HttpResponse::Ok().json(event)
}

#[delete("/events/{event_id}")]
pub async fn delete_event(
    path: web::Path<(Uuid,)>,
    data: web::Data<MyData>,
    session: Session,
) -> impl Responder {
    let user_id: Uuid;
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

    let mut conn: PgPooledConnection = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let data = db::get_event_by_id(&mut conn, path.0, null_uuid);
    let event: EventWithMembers;
    match data{
        Some(e) => {
            event = e;
        }
        None => {
            return HttpResponse::NotFound().json(DefaultError {
                message: "Not Found".to_string(),
                error_code: "404".to_string(),
            });
        }
    }

    if event.user_id!= user_id {
        return HttpResponse::Forbidden().json(DefaultError {
            message: "Forbidden".to_string(),
            error_code: "403".to_string(),
        });
    }

    let result = db::delete_event(&mut conn, path.0);
    if result == false {
        return HttpResponse::InternalServerError().json(DefaultError {
            message: "Failed to delete event".to_string(),
            error_code: "500".to_string(),
        });
    }

    HttpResponse::Ok().json(DefaultMsg{
        message: "Event deleted".to_string(),
        message_code: "200".to_string(),
    })
}

#[get("/users/{user_id}/events")]
pub async fn get_user_events(
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

    let mut conn: PgPooledConnection = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    if path.0 != user_id {
        return HttpResponse::Forbidden().json(DefaultError {
            message: "Forbidden".to_string(),
            error_code: "403".to_string(),
        });
    }

    let events = db::get_events_by_user_id(&mut conn, user_id);

    HttpResponse::Ok().json(events)
}