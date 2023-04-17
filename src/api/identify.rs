use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use jsonwebtoken_google::Parser;
use serde::{Deserialize, Serialize};
use actix_session::Session;

use crate::MyData;
use crate::db::get_or_create_user;
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
                return HttpResponse::Ok().body(format!("Already logged in as {}", user_id));
            }
        }
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get session"),
    }

    match req.cookie("g_csrf_token") {
        Some(c) => {
            g_csrf_token = c.value().to_string();
        }
        None => {
            g_csrf_token = "".to_string();
        }
    }

    if g_csrf_token != form.g_csrf_token {
        return HttpResponse::BadRequest().body("CSRF token mismatch");
    }

    let parser = Parser::new(&data.google_client_id);
    let claims = parser.parse::<TokenClaims>(&form.credential).await.unwrap();

    let mut conn: PgPooledConnection = data.pool.get().expect("couldn't get db connection from pool");
    let user = get_or_create_user(&mut conn, claims.sub, claims.name, claims.email, claims.picture);

    match session.insert("user_id", user.id) {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to set session"),
    }

    HttpResponse::Ok().json(user)
}
