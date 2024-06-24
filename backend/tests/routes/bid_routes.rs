use actix_web::{test, App};
use backend::routes::bid;
use serde_json::json;
use std::env;
use once_cell::sync::Lazy;
use crate::helpers::{clean_up_database, create_user, create_repair_request, create_repair_shop, establish_connection, create_bid, INIT};

#[actix_rt::test]
async fn test_create_bid() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);
    let repair_request = create_repair_request(conn, user.id);
    let repair_shop = create_repair_shop(conn);

    let app = test::init_service(App::new().configure(bid::init)).await;
    let req = test::TestRequest::post()
        .uri("/bid")
        .set_json(json!({
            "repair_request_id": repair_request.id,
            "repair_shop_id": repair_shop.id,
            "bid_amount": 100.0,
            "status": "pending"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to create bid: status: {}, body: {:?}", status, body);
    }
    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_get_bid() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);
    let repair_request = create_repair_request(conn, user.id);
    let repair_shop = create_repair_shop(conn);
    let bid = create_bid(conn, repair_request.id, repair_shop.id);

    let app = test::init_service(App::new().configure(bid::init)).await;
    let req = test::TestRequest::get()
        .uri(&format!("/bid/{}", bid.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to get bid: status: {}, body: {:?}", status, body);
    }
    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_update_bid() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);
    let repair_request = create_repair_request(conn, user.id);
    let repair_shop = create_repair_shop(conn);
    let bid = create_bid(conn, repair_request.id, repair_shop.id);

    let app = test::init_service(App::new().configure(bid::init)).await;
    let req = test::TestRequest::put()
        .uri(&format!("/bid/{}", bid.id))
        .set_json(json!({
            "repair_request_id": repair_request.id.to_string(),
            "repair_shop_id": repair_shop.id.to_string(),
            "bid_amount": 150.0,
            "status": "accepted"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to update bid: status: {}, body: {:?}", status, body);
    }
    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_delete_bid() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);
    let repair_request = create_repair_request(conn, user.id);
    let repair_shop = create_repair_shop(conn);
    let bid = create_bid(conn, repair_request.id, repair_shop.id);

    let app = test::init_service(App::new().configure(bid::init)).await;
    let req = test::TestRequest::delete()
        .uri(&format!("/bid/{}", bid.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to delete bid: status: {}, body: {:?}", status, body);
    }
    assert!(status.is_success());
}

