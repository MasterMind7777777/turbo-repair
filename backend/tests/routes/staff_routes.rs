use std::env;
use actix_web::{test, App};
use backend::routes::staff;
use once_cell::sync::Lazy;
use crate::helpers::{clean_up_database, create_user, create_repair_shop, create_staff, establish_connection, INIT};

#[actix_rt::test]
async fn test_create_staff() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);
    let repair_shop = create_repair_shop(conn);

    let app = test::init_service(App::new().configure(staff::init)).await;
    let req = test::TestRequest::post()
        .uri("/staff")
        .set_json(serde_json::json!({
            "user_id": user.id.to_string(),
            "repair_shop_id": repair_shop.id.to_string(),
            "role": "manager"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to create staff. Status: {:?}, Body: {:?}", status, body);
    }

    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_get_staff() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);
    let repair_shop = create_repair_shop(conn);
    let staff = create_staff(conn, user.id, repair_shop.id);

    let app = test::init_service(App::new().configure(staff::init)).await;
    let req = test::TestRequest::get()
        .uri(&format!("/staff/{}", staff.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to get staff. Status: {:?}, Body: {:?}", status, body);
    }

    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_update_staff() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);
    let repair_shop = create_repair_shop(conn);
    let staff = create_staff(conn, user.id, repair_shop.id);

    let app = test::init_service(App::new().configure(staff::init)).await;
    let req = test::TestRequest::put()
        .uri(&format!("/staff/{}", staff.id))
        .set_json(serde_json::json!({
            "user_id": user.id.to_string(),
            "repair_shop_id": repair_shop.id.to_string(),
            "role": "updated_role"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to update staff. Status: {:?}, Body: {:?}", status, body);
    }

    assert!(status.is_success());
}

#[actix_rt::test]
async fn test_delete_staff() {
    let _ = Lazy::force(&INIT);
    let conn = &mut establish_connection().get().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Using database: {}", database_url);
    clean_up_database(conn);
    let user = create_user(conn);
    let repair_shop = create_repair_shop(conn);
    let staff = create_staff(conn, user.id, repair_shop.id);

    let app = test::init_service(App::new().configure(staff::init)).await;
    let req = test::TestRequest::delete()
        .uri(&format!("/staff/{}", staff.id))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    let body = test::read_body(resp).await;

    if !status.is_success() {
        panic!("Failed to delete staff. Status: {:?}, Body: {:?}", status, body);
    }

    assert!(status.is_success());
}

