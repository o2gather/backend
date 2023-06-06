#[test]
fn test_get_or_create_user() {
    use crate::db::get_or_create_user;
    use diesel::pg::PgConnection;
    use diesel::Connection;
    use dotenvy;
    use std::env;

    dotenvy::from_filename(".env.test").ok();

    let mut conn =
        PgConnection::establish(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .unwrap();
    let user = get_or_create_user(
        &mut conn,
        "test_get_or_create_user".to_string(),
        "test_user".to_string(),
        "a".to_string(),
        "a".to_string(),
    );
    assert_eq!(user.name, "test_user");
}

#[test]
fn test_get_user_by_id() {
    use crate::db::get_or_create_user;
    use crate::db::get_user_by_id;
    use diesel::pg::PgConnection;
    use diesel::Connection;
    use dotenvy;
    use std::env;

    dotenvy::from_filename(".env.test").ok();

    let mut conn =
        PgConnection::establish(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .unwrap();
    let user = get_or_create_user(
        &mut conn,
        "test_get_user_by_id".to_string(),
        "test_user".to_string(),
        "a".to_string(),
        "a".to_string(),
    );
    
    let user2 = get_user_by_id(&mut conn, user.id);
    assert_eq!(user2.name, "test_user");
    assert_eq!(user2.id, user.id);
}

#[test]
fn test_update_user() {
    use crate::db::get_or_create_user;
    use crate::db::get_user_by_id;
    use crate::db::update_user;
    use crate::models::UpdateUser;
    use diesel::pg::PgConnection;
    use diesel::Connection;
    use dotenvy;
    use std::env;

    dotenvy::from_filename(".env.test").ok();

    let mut conn =
        PgConnection::establish(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .unwrap();
    let user = get_or_create_user(
        &mut conn,
        "test_update_user".to_string(),
        "test_user".to_string(),
        "a".to_string(),
        "a".to_string(),
    );
    let user_data : UpdateUser = UpdateUser {
        name: Some("test_user2".to_string()),
        email: None,
        phone: None,
        avatar: None,
    };

    update_user(&mut conn, user.id, user_data);
    
    let user2 = get_user_by_id(&mut conn, user.id);
    assert_eq!(user2.name, "test_user2");
    assert_eq!(user2.id, user.id);
}

#[test]
fn test_create_event() {
    use crate::db::get_or_create_user;
    use crate::db::create_event;
    use crate::db::delete_event;
    use crate::models::NewEvent;
    use diesel::pg::PgConnection;
    use diesel::Connection;
    use chrono::*;
    use dotenvy;
    use std::env;

    dotenvy::from_filename(".env.test").ok();
    let mut conn =
        PgConnection::establish(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .unwrap();
    let user = get_or_create_user(
        &mut conn,
        "test_create_event".to_string(),
        "test_user".to_string(),
        "a".to_string(),
        "a".to_string(),
    );
    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    let event_data = NewEvent { 
        name: "test_event".to_string(),
        description: "test_event".to_string(),
        category: "test_event".to_string(),
        start_time: NaiveDateTime::new(d, t),
        end_time: NaiveDateTime::new(d, t),
        user_id: user.id,
        max_amount: 10,
        min_amount: 1,
     };

    let data = create_event(&mut conn, event_data);
    delete_event(&mut conn, data.id);

    assert_eq!(data.name, "test_event");
    assert_eq!(data.description, "test_event");
}

#[test]
fn test_get_events() {
    use crate::db::get_or_create_user;
    use crate::db::create_event;
    use crate::db::get_events;
    use crate::db::delete_event;
    use crate::models::NewEvent;
    use diesel::pg::PgConnection;
    use diesel::Connection;
    use chrono::*;
    use dotenvy;
    use std::env;

    dotenvy::from_filename(".env.test").ok();
    let mut conn =
        PgConnection::establish(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .unwrap();
    let user = get_or_create_user(
        &mut conn,
        "test_get_events".to_string(),
        "test_user".to_string(),
        "a".to_string(),
        "a".to_string(),
    );
    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    let event_data = NewEvent { 
        name: "test_event".to_string(),
        description: "test_event".to_string(),
        category: "test_event".to_string(),
        start_time: NaiveDateTime::new(d, t),
        end_time: NaiveDateTime::new(d, t),
        user_id: user.id,
        max_amount: 10,
        min_amount: 1,
     };
     

    let data =create_event(&mut conn, event_data);
    let events = get_events(&mut conn);
    delete_event(&mut conn, data.id);

    assert!(events.len() > 0);
}

#[test]
fn test_get_event_by_id() {
    use crate::db::get_or_create_user;
    use crate::db::create_event;
    use crate::db::get_event_by_id;
    use crate::db::delete_event;
    use crate::models::NewEvent;
    use diesel::pg::PgConnection;
    use diesel::Connection;
    use chrono::*;
    use dotenvy;
    use std::env;
    use uuid::Uuid;

    dotenvy::from_filename(".env.test").ok();
    let mut conn =
        PgConnection::establish(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .unwrap();
    let user = get_or_create_user(
        &mut conn,
        "test_create_event".to_string(),
        "test_user".to_string(),
        "a".to_string(),
        "a".to_string(),
    );
    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    let event_data = NewEvent { 
        name: "test_event".to_string(),
        description: "test_event".to_string(),
        category: "test_event".to_string(),
        start_time: NaiveDateTime::new(d, t),
        end_time: NaiveDateTime::new(d, t),
        user_id: user.id,
        max_amount: 10,
        min_amount: 1,
     };

    let data = create_event(&mut conn, event_data);
    let event = get_event_by_id(&mut conn, data.id, Uuid::nil());
    assert!(event.is_some());
    let event = event.unwrap();
    delete_event(&mut conn, data.id);

    assert_eq!(event.name, data.name);
    assert_eq!(event.description, data.description);
}

#[test]
fn test_delete_event() {
    use crate::db::get_or_create_user;
    use crate::db::create_event;
    use crate::db::get_event_by_id;
    use crate::db::delete_event;
    use crate::models::NewEvent;
    use diesel::pg::PgConnection;
    use diesel::Connection;
    use chrono::*;
    use dotenvy;
    use std::env;
    use uuid::Uuid;

    dotenvy::from_filename(".env.test").ok();
    let mut conn =
        PgConnection::establish(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .unwrap();
    let user = get_or_create_user(
        &mut conn,
        "test_delete_event".to_string(),
        "test_user".to_string(),
        "a".to_string(),
        "a".to_string(),
    );
    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    let event_data = NewEvent { 
        name: "test_event".to_string(),
        description: "test_event".to_string(),
        category: "test_event".to_string(),
        start_time: NaiveDateTime::new(d, t),
        end_time: NaiveDateTime::new(d, t),
        user_id: user.id,
        max_amount: 10,
        min_amount: 1,
     };

    let data = create_event(&mut conn, event_data);
    delete_event(&mut conn, data.id);

    let event = get_event_by_id(&mut conn, data.id, Uuid::nil());
    assert!(event.is_none());
}
