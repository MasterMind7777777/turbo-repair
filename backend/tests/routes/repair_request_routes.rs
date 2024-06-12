use actix_web::{test, App};
use backend::routes::repair_request;
use dotenv::dotenv;

#[actix_rt::test]
async fn test_create_repair_request() {
    dotenv().ok();  // Load environment variables from .env file

    let app = test::init_service(App::new().configure(repair_request::init)).await;
    let req = test::TestRequest::post()
        .uri("/repair_request")
        .set_json(serde_json::json!({
            "customer_id": "some-customer-id",
            "description": "Test description"
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
async fn test_delete_repair_request() {
    dotenv().ok();  // Load environment variables from .env file

    let app = test::init_service(App::new().configure(repair_request::init)).await;
    let req = test::TestRequest::delete()
        .uri("/repair_request/some-id")
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    if !status.is_success() {
        let body = test::read_body(resp).await;
        panic!("Request failed with status: {}, body: {:?}", status, body);
    }
}

