use actix_web::{test, App};
use backend::routes::auth;
use std::env;
use once_cell::sync::Lazy;
use crate::helpers::{clean_up_database, establish_connection, INIT};

#[actix_rt::test]
async fn test_register_user() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);

    let app = test::init_service(App::new().configure(auth::init)).await;
    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(serde_json::json!({
            "username": "testuser",
            "email": format!("testuser+{}@example.com", uuid::Uuid::new_v4()),
            "password": "password123"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Request failed with status: {}, body: {:?}", status, body);
    }
    assert!(status.is_success());
}
