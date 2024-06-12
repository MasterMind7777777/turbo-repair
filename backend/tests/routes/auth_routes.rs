use actix_web::{test, App};
use backend::routes::auth;
use dotenv::dotenv;

#[actix_rt::test]
async fn test_register_user() {
    dotenv().ok();  // Load environment variables from .env file

    let app = test::init_service(App::new().configure(auth::init)).await;
    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(serde_json::json!({
            "username": "testuser",
            "email": "testuser@example.com",
            "password": "password123"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    if !status.is_success() {
        let body = test::read_body(resp).await;
        panic!("Request failed with status: {}, body: {:?}", status, body);
    }
}

