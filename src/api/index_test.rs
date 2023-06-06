#[cfg(test)]
use crate::api::init;
#[cfg(test)]
use actix_web::{test, App};

#[actix_web::test]
async fn test_health() {
    let app = test::init_service(
        App::new().configure(init),
    ).await;

    let req = test::TestRequest::get().uri("/ping").to_request();
    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    assert!(status.is_success());
}