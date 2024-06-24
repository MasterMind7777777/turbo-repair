use actix_web::{test, App};
use backend::routes::user;
use once_cell::sync::Lazy;
use crate::helpers::{clean_up_database, create_user, establish_connection, INIT};
use std::env;

#[actix_rt::test]
async fn test_get_user_details() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);

    let app = test::init_service(App::new().configure(user::init)).await;
    let req = test::TestRequest::get()
        .uri(&format!("/user/{}", user.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to get user details: status: {}, body: {:?}", status, body);
    }
    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_update_user_profile() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);

    let app = test::init_service(App::new().configure(user::init)).await;
    let req = test::TestRequest::put()
        .uri(&format!("/user/{}", user.id))
        .set_json(serde_json::json!({
            "email": "newemail@example.com",
            "password": "newpassword"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to update user profile: status: {}, body: {:?}", status, body);
    }
    assert!(status.is_success());
}
