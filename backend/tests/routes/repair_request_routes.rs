use std::env;
use actix_web::{test, App};
use backend::routes::repair_request;
use once_cell::sync::Lazy;
use crate::helpers::{clean_up_database, create_user, establish_connection, create_repair_request, INIT};

#[actix_rt::test]
async fn test_create_repair_request() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);

    let app = test::init_service(App::new().configure(repair_request::init)).await;
    let req = test::TestRequest::post()
        .uri("/repair_request")
        .set_json(serde_json::json!({
            "customer_id": user.id.to_string(),
            "description": "Test description"
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

#[actix_rt::test]
async fn test_delete_repair_request() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);
    let repair_request = create_repair_request(conn, user.id);

    let app = test::init_service(App::new().configure(repair_request::init)).await;
    let req = test::TestRequest::delete()
        .uri(&format!("/repair_request/{}", repair_request.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Request failed with status: {}, body: {:?}", status, body);
    }

    assert!(status.is_success());
}

