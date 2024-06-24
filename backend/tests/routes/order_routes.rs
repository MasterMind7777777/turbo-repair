use std::env;
use actix_web::{test, App};
use backend::routes::order;
use once_cell::sync::Lazy;
use crate::helpers::{clean_up_database, create_order, create_repair_request, create_repair_shop, create_user, establish_connection, INIT};

#[actix_rt::test]
async fn test_create_order() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    // Log the database URL being used
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);
    let repair_request = create_repair_request(conn, user.id);
    let repair_shop = create_repair_shop(conn);

    let app = test::init_service(App::new().configure(order::init)).await;
    let req = test::TestRequest::post()
        .uri("/order")
        .set_json(serde_json::json!({
            "repair_request_id": repair_request.id.to_string(),
            "repair_shop_id": repair_shop.id.to_string(),
            "status": "ongoing"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Log the response status and body for debugging
    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to create order. Status: {:?}, Body: {:?}", status, body);
    }

    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_get_order() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    // Log the database URL being used
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);
    let repair_request = create_repair_request(conn, user.id);
    let repair_shop = create_repair_shop(conn);
    let order = create_order(conn, repair_request.id, repair_shop.id);

    let app = test::init_service(App::new().configure(order::init)).await;
    let req = test::TestRequest::get()
        .uri(&format!("/order/{}", order.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Log the response status and body for debugging
    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to get order. Status: {:?}, Body: {:?}", status, body);
    }

    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_update_order() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    // Log the database URL being used
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);
    let repair_request = create_repair_request(conn, user.id);
    let repair_shop = create_repair_shop(conn);
    let order = create_order(conn, repair_request.id, repair_shop.id);

    let app = test::init_service(App::new().configure(order::init)).await;
    let req = test::TestRequest::put()
        .uri(&format!("/order/{}", order.id))
        .set_json(serde_json::json!({
            "repair_request_id": repair_request.id.to_string(),
            "repair_shop_id": repair_shop.id.to_string(),
            "status": "completed"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Log the response status and body for debugging
    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to update order. Status: {:?}, Body: {:?}", status, body);
    }

    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_delete_order() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    // Log the database URL being used
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);
    let repair_request = create_repair_request(conn, user.id);
    let repair_shop = create_repair_shop(conn);
    let order = create_order(conn, repair_request.id, repair_shop.id);

    let app = test::init_service(App::new().configure(order::init)).await;
    let req = test::TestRequest::delete()
        .uri(&format!("/order/{}", order.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    // Log the response status and body for debugging
    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to delete order. Status: {:?}, Body: {:?}", status, body);
    }

    assert!(status.is_success());
}
