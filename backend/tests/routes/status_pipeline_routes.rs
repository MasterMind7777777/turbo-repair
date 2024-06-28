use std::env;
use actix_web::{test, App};
use backend::routes::status_pipeline;
use once_cell::sync::Lazy;
use crate::helpers::{clean_up_database, create_order, create_repair_request, create_user, create_repair_shop, create_status_pipeline, establish_connection, INIT};

#[actix_rt::test]
async fn test_create_status_pipeline() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);

    let user = create_user(conn);
    let repair_shop = create_repair_shop(conn);
    let repair_request = create_repair_request(conn, user.id);
    let order = create_order(conn, repair_request.id, repair_shop.id);

    let app = test::init_service(App::new().configure(status_pipeline::init)).await;
    let req = test::TestRequest::post()
        .uri("/status_pipeline")
        .set_json(serde_json::json!({
            "order_id": order.id.to_string(),
            "status": "in_progress"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to create status pipeline. Status: {:?}, Body: {:?}", status, body);
    }

    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_get_status_pipeline() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);

    let user = create_user(conn);
    let repair_shop = create_repair_shop(conn);
    let repair_request = create_repair_request(conn, user.id);
    let order = create_order(conn, repair_request.id, repair_shop.id);
    let status_pipeline = create_status_pipeline(conn, order.id);

    let app = test::init_service(App::new().configure(status_pipeline::init)).await;
    let req = test::TestRequest::get()
        .uri(&format!("/status_pipeline/{}", status_pipeline.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to get status pipeline. Status: {:?}, Body: {:?}", status, body);
    }

    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_update_status_pipeline() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);

    let user = create_user(conn);
    let repair_shop = create_repair_shop(conn);
    let repair_request = create_repair_request(conn, user.id);
    let order = create_order(conn, repair_request.id, repair_shop.id);
    let status_pipeline = create_status_pipeline(conn, order.id);

    let app = test::init_service(App::new().configure(status_pipeline::init)).await;
    let req = test::TestRequest::put()
        .uri(&format!("/status_pipeline/{}", status_pipeline.id))
        .set_json(serde_json::json!({
            "order_id": order.id.to_string(),
            "status": "completed"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to update status pipeline. Status: {:?}, Body: {:?}", status, body);
    }

    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_delete_status_pipeline() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);

    let user = create_user(conn);
    let repair_shop = create_repair_shop(conn);
    let repair_request = create_repair_request(conn, user.id);
    let order = create_order(conn, repair_request.id, repair_shop.id);
    let status_pipeline = create_status_pipeline(conn, order.id);

    let app = test::init_service(App::new().configure(status_pipeline::init)).await;
    let req = test::TestRequest::delete()
        .uri(&format!("/status_pipeline/{}", status_pipeline.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to delete status pipeline. Status: {:?}, Body: {:?}", status, body);
    }

    assert!(status.is_success());
}

