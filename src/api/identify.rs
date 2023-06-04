use actix_session::Session;
use actix_web::cookie::Cookie;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use jsonwebtoken_google::Parser;
use serde::{Deserialize, Serialize};

use crate::api::types::{DefaultError, DefaultMsg};
use crate::db::get_or_create_user;
use crate::MyData;
use crate::PgPooledConnection;

#[derive(Deserialize)]
pub struct LoginFormData {
    credential: String,
    g_csrf_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct TokenClaims {
    sub: String,
    email: String,
    name: String,
    picture: String,
}

#[post("/login")]
pub async fn user_login(
    req: HttpRequest,
    data: web::Data<MyData>,
    form: web::Form<LoginFormData>,
    session: Session,
) -> impl Responder {
    let g_csrf_token: String;

    match session.get::<String>("user_id") {
        Ok(user_id) => {
            if let Some(user_id) = user_id {
                return HttpResponse::Conflict().json(DefaultError {
                    message: format!("Already logged in as {}", user_id),
                    error_code: "409".to_string(),
                });
            }
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(DefaultError {
                message: "Failed to get session".to_string(),
                error_code: "500".to_string(),
            })
        }
    }

    match req.cookie("g_csrf_token") {
        Some(c) => {
            g_csrf_token = c.value().to_string();
        }
        None => {
            g_csrf_token = "".to_string();
        }
    }

    if data.cors_enabled == false && g_csrf_token != form.g_csrf_token {
        return HttpResponse::Unauthorized().json(DefaultError {
            message: "Invalid CSRF token".to_string(),
            error_code: "401".to_string(),
        });
    }

    let parser = Parser::new(&data.google_client_id);
    let claims = parser.parse::<TokenClaims>(&form.credential).await.unwrap();

    let mut conn: PgPooledConnection = data
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let user = get_or_create_user(
        &mut conn,
        claims.sub,
        claims.name,
        claims.email,
        claims.picture,
    );

    match session.insert("user_id", user.id) {
        Ok(_) => (),
        Err(_) => {
            return HttpResponse::InternalServerError().json(DefaultError {
                message: "Failed to set session".to_string(),
                error_code: "500".to_string(),
            })
        }
    }
    let cookie = Cookie::build("user_id", user.id.to_string())
        .path("/")
        .secure(false)
        .http_only(false)
        .finish();

    HttpResponse::SeeOther()
        .append_header(("Location", data.redirect_url.clone()))
        .cookie(cookie)
        .finish()
}

#[post("/logout")]
pub async fn user_logout(session: Session) -> impl Responder {
    session.purge();

    HttpResponse::Ok().json(DefaultMsg {
        message: "Logged out".to_string(),
        message_code: "200".to_string(),
    })
}
