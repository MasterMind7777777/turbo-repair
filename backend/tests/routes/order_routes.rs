use actix_web::{test, App};
use backend::routes::order;

#[actix_rt::test]
async fn test_create_order() {
    let  app = test::init_service(App::new().configure(order::init)).await;
    let req = test::TestRequest::post()
        .uri("/order")
        .set_json(serde_json::json!({
            "repair_request_id": "some-uuid",
            "repair_shop_id": "some-uuid",
            "status": "ongoing"
        }))
        .to_request();
    let resp = test::call_service(& app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_order() {
    let  app = test::init_service(App::new().configure(order::init)).await;
    let req = test::TestRequest::get().uri("/order/some-uuid").to_request();
    let resp = test::call_service(& app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_update_order() {
    let  app = test::init_service(App::new().configure(order::init)).await;
    let req = test::TestRequest::put()
        .uri("/order/some-uuid")
        .set_json(serde_json::json!({
            "status": "completed"
        }))
        .to_request();
    let resp = test::call_service(& app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_delete_order() {
    let  app = test::init_service(App::new().configure(order::init)).await;
    let req = test::TestRequest::delete().uri("/order/some-uuid").to_request();
    let resp = test::call_service(& app, req).await;
    assert!(resp.status().is_success());
}

