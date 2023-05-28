#[cfg(test)]
use crate::api::init;
#[cfg(test)]
use actix_web::{test, App};


#[actix_web::test]
async fn test_logout_without_session() {
    let app = test::init_service(
        App::new().configure(init),
    ).await;

    let req = test::TestRequest::post().uri("/api/v1/logout").to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    assert!(status.is_client_error());
}

#[actix_web::test]
async fn test_logout_with_session() {
    use actix_web::cookie::Cookie;
    use actix_session::{SessionMiddleware, storage::CookieSessionStore};
    use actix_web::cookie::Key;


    let app = test::init_service(
        App::new().wrap(
            SessionMiddleware::builder(
                CookieSessionStore::default(),
                Key::from(str::repeat("a", 64).as_bytes())
            )
            .cookie_name("session".to_string())
            .build()
        ).configure(init),
    ).await;
    let req = test::TestRequest::post().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    let headers = resp.headers();
    let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
    let parsed_cookie = Cookie::parse_encoded(cookie_header).unwrap();

    let status = resp.status();
    assert!(status.is_success());


    let req = test::TestRequest::post().uri("/api/v1/logout").cookie(
        parsed_cookie
    ).to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    assert!(status.is_success());
}