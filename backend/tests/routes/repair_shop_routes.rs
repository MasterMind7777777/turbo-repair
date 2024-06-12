use actix_web::{test, App};
use backend::routes::repair_shop;
use dotenv::dotenv;

#[actix_rt::test]
async fn test_create_repair_shop() {
    dotenv().ok();  // Load environment variables from .env file

    let app = test::init_service(App::new().configure(repair_shop::init)).await;
    let req = test::TestRequest::post()
        .uri("/repair_shop")
        .set_json(serde_json::json!({
            "name": "Test Repair Shop",
            "address_id": "some-address-id"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    if !status.is_success() {
        let body = test::read_body(resp).await;
        panic!("Request failed with status: {}, body: {:?}", status, body);
    }
}

#[actix_rt::test]
async fn test_delete_repair_shop() {
    dotenv().ok();  // Load environment variables from .env file

    let app = test::init_service(App::new().configure(repair_shop::init)).await;
    let req = test::TestRequest::delete()
        .uri("/repair_shop/some-id")
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    if !status.is_success() {
        let body = test::read_body(resp).await;
        panic!("Request failed with status: {}, body: {:?}", status, body);
    }
}

#[actix_rt::test]
async fn test_get_repair_shop() {
    dotenv().ok();  // Load environment variables from .env file

    let app = test::init_service(App::new().configure(repair_shop::init)).await;
    let req = test::TestRequest::get()
        .uri("/repair_shop/some-id")
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    if !status.is_success() {
        let body = test::read_body(resp).await;
        panic!("Request failed with status: {}, body: {:?}", status, body);
    }
}

#[actix_rt::test]
async fn test_update_repair_shop() {
    dotenv().ok();  // Load environment variables from .env file

    let app = test::init_service(App::new().configure(repair_shop::init)).await;
    let req = test::TestRequest::put()
        .uri("/repair_shop/some-id")
        .set_json(serde_json::json!({
            "name": "Updated Repair Shop",
            "address_id": "updated-address-id"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    if !status.is_success() {
        let body = test::read_body(resp).await;
        panic!("Request failed with status: {}, body: {:?}", status, body);
    }
}

