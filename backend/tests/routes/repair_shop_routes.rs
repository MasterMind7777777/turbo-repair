use backend::routes::address;
use std::env;
use actix_web::{test, App};
use backend::routes::repair_shop;
use once_cell::sync::Lazy;
use crate::helpers::{clean_up_database, create_address, create_repair_shop, establish_connection, INIT};

#[actix_rt::test]
async fn test_create_repair_shop() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let repair_shop = create_repair_shop(conn);
    let address = create_address(conn, repair_shop.id);

    let app = test::init_service(App::new().configure(repair_shop::init)).await;
    let req = test::TestRequest::post()
        .uri("/repair_shop")
        .set_json(serde_json::json!({
            "name": "Test Repair Shop",
            "address_id": address.id.to_string()
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    if !status.is_success() {
        let body = test::read_body(resp).await;
        panic!("Request failed with status: {}, body: {:?}", status, body);
    }

    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_delete_repair_shop() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let repair_shop = create_repair_shop(conn);

    let app = test::init_service(App::new().configure(repair_shop::init)).await;
    let req = test::TestRequest::delete()
        .uri(&format!("/repair_shop/{}", repair_shop.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    if !status.is_success() {
        let body = test::read_body(resp).await;
        panic!("Request failed with status: {}, body: {:?}", status, body);
    }

    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_get_repair_shop() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let repair_shop = create_repair_shop(conn);

    let app = test::init_service(App::new().configure(repair_shop::init)).await;
    let req = test::TestRequest::get()
        .uri(&format!("/repair_shop/{}", repair_shop.id))
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
async fn test_update_repair_shop() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let repair_shop = create_repair_shop(conn);
    let address = create_address(conn, repair_shop.id);

    let app = test::init_service(App::new().configure(repair_shop::init)).await;
    let req = test::TestRequest::put()
        .uri(&format!("/repair_shop/{}", repair_shop.id))
        .set_json(serde_json::json!({
            "name": "Updated Repair Shop",
            "address_id": address.id.to_string()
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
async fn test_create_address() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let repair_shop = create_repair_shop(conn);

    let app = test::init_service(App::new().configure(repair_shop::init).configure(address::init)).await;
    let req = test::TestRequest::post()
        .uri("/address")
        .set_json(serde_json::json!({
            "repair_shop_id": repair_shop.id.to_string(),
            "street": "123 Main St",
            "city": "Test City",
            "state": "Test State",
            "zip": "12345",
            "country": "Test Country"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    if !status.is_success() {
        let body = test::read_body(resp).await;
        panic!("Request failed with status: {}, body: {:?}", status, body);
    }

    assert!(status.is_success());
}
