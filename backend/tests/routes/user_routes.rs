use actix_web::{test, App};
use backend::routes::user;
use dotenv::dotenv;

#[actix_rt::test]
async fn test_get_user_details() {
    dotenv().ok();  // Load environment variables from .env file

    let app = test::init_service(App::new().configure(user::init)).await;
    let req = test::TestRequest::get().uri("/user/some-uuid").to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    if !status.is_success() {
        let body = test::read_body(resp).await;
        panic!("Request failed with status: {}, body: {:?}", status, body);
    }
}

#[actix_rt::test]
async fn test_update_user_profile() {
    dotenv().ok();  // Load environment variables from .env file

    let app = test::init_service(App::new().configure(user::init)).await;
    let req = test::TestRequest::put()
        .uri("/user/some-uuid")
        .set_json(serde_json::json!({
            "email": "newemail@example.com",
            "password": "newpassword"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;

    let status = resp.status();
    if !status.is_success() {
        let body = test::read_body(resp).await;
        panic!("Request failed with status: {}, body: {:?}", status, body);
    }
}

