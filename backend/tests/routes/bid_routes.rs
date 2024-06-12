use actix_web::{test, App};
use backend::{models::{bid::Bid, repair_request::RepairRequest, repair_shop::RepairShop}, routes::bid};
use serde_json::json;

#[actix_rt::test]
async fn test_create_bid() {
    let app = test::init_service(App::new().configure(bid::init)).await;

    // Create repair request first
    let repair_request_resp = test::TestRequest::post()
        .uri("/repair_request")
        .set_json(json!({
            "details": "Test repair request"
        }))
        .to_request();
    let repair_request_resp = test::call_service(&app, repair_request_resp).await;
    let repair_request_status = repair_request_resp.status();
    let repair_request_body = test::read_body(repair_request_resp).await;
    if !repair_request_status.is_success() {
        panic!("Failed to create repair request: status: {}, body: {:?}", repair_request_status, repair_request_body);
    }
    let repair_request: RepairRequest = serde_json::from_slice(&repair_request_body).unwrap();

    // Create repair shop first
    let repair_shop_resp = test::TestRequest::post()
        .uri("/repair_shop")
        .set_json(json!({
            "name": "Test Repair Shop"
        }))
        .to_request();
    let repair_shop_resp = test::call_service(&app, repair_shop_resp).await;
    let repair_shop_status = repair_shop_resp.status();
    let repair_shop_body = test::read_body(repair_shop_resp).await;
    if !repair_shop_status.is_success() {
        panic!("Failed to create repair shop: status: {}, body: {:?}", repair_shop_status, repair_shop_body);
    }
    let repair_shop: RepairShop = serde_json::from_slice(&repair_shop_body).unwrap();

    // Create bid using the above UUIDs
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
    let resp_status = resp.status();
    let resp_body = test::read_body(resp).await;
    if !resp_status.is_success() {
        panic!("Failed to create bid: status: {}, body: {:?}", resp_status, resp_body);
    }
    assert!(resp_status.is_success());
}

#[actix_rt::test]
async fn test_get_bid() {
    let app = test::init_service(App::new().configure(bid::init)).await;

    // Create repair request first
    let repair_request_resp = test::TestRequest::post()
        .uri("/repair_request")
        .set_json(json!({
            "details": "Test repair request"
        }))
        .to_request();
    let repair_request_resp = test::call_service(&app, repair_request_resp).await;
    let repair_request_status = repair_request_resp.status();
    let repair_request_body = test::read_body(repair_request_resp).await;
    if !repair_request_status.is_success() {
        panic!("Failed to create repair request: status: {}, body: {:?}", repair_request_status, repair_request_body);
    }
    let repair_request: RepairRequest = serde_json::from_slice(&repair_request_body).unwrap();

    let repair_shop_resp = test::TestRequest::post()
        .uri("/repair_shop")
        .set_json(json!({
            "name": "Test Repair Shop"
        }))
        .to_request();
    let repair_shop_resp = test::call_service(&app, repair_shop_resp).await;
    let repair_shop_status = repair_shop_resp.status();
    let repair_shop_body = test::read_body(repair_shop_resp).await;
    if !repair_shop_status.is_success() {
        panic!("Failed to create repair shop: status: {}, body: {:?}", repair_shop_status, repair_shop_body);
    }
    let repair_shop: RepairShop = serde_json::from_slice(&repair_shop_body).unwrap();

    let create_bid_req = test::TestRequest::post()
        .uri("/bid")
        .set_json(json!({
            "repair_request_id": repair_request.id,
            "repair_shop_id": repair_shop.id,
            "bid_amount": 100.0,
            "status": "pending"
        }))
        .to_request();
    let create_bid_resp = test::call_service(&app, create_bid_req).await;
    let create_bid_status = create_bid_resp.status();
    let create_bid_body = test::read_body(create_bid_resp).await;
    if !create_bid_status.is_success() {
        panic!("Failed to create bid: status: {}, body: {:?}", create_bid_status, create_bid_body);
    }
    let bid: Bid = serde_json::from_slice(&create_bid_body).unwrap();

    // Get the created bid
    let get_req = test::TestRequest::get()
        .uri(&format!("/bid/{}", bid.id))
        .to_request();
    let resp = test::call_service(&app, get_req).await;
    let resp_status = resp.status();
    let resp_body = test::read_body(resp).await;
    if !resp_status.is_success() {
        panic!("Failed to get bid: status: {}, body: {:?}", resp_status, resp_body);
    }
    assert!(resp_status.is_success());
}

#[actix_rt::test]
async fn test_update_bid() {
    let app = test::init_service(App::new().configure(bid::init)).await;

    // Create repair request first
    let repair_request_resp = test::TestRequest::post()
        .uri("/repair_request")
        .set_json(json!({
            "details": "Test repair request"
        }))
        .to_request();
    let repair_request_resp = test::call_service(&app, repair_request_resp).await;
    let repair_request_status = repair_request_resp.status();
    let repair_request_body = test::read_body(repair_request_resp).await;
    if !repair_request_status.is_success() {
        panic!("Failed to create repair request: status: {}, body: {:?}", repair_request_status, repair_request_body);
    }
    let repair_request: RepairRequest = serde_json::from_slice(&repair_request_body).unwrap();

    let repair_shop_resp = test::TestRequest::post()
        .uri("/repair_shop")
        .set_json(json!({
            "name": "Test Repair Shop"
        }))
        .to_request();
    let repair_shop_resp = test::call_service(&app, repair_shop_resp).await;
    let repair_shop_status = repair_shop_resp.status();
    let repair_shop_body = test::read_body(repair_shop_resp).await;
    if !repair_shop_status.is_success() {
        panic!("Failed to create repair shop: status: {}, body: {:?}", repair_shop_status, repair_shop_body);
    }
    let repair_shop: RepairShop = serde_json::from_slice(&repair_shop_body).unwrap();

    let create_bid_req = test::TestRequest::post()
        .uri("/bid")
        .set_json(json!({
            "repair_request_id": repair_request.id,
            "repair_shop_id": repair_shop.id,
            "bid_amount": 100.0,
            "status": "pending"
        }))
        .to_request();
    let create_bid_resp = test::call_service(&app, create_bid_req).await;
    let create_bid_status = create_bid_resp.status();
    let create_bid_body = test::read_body(create_bid_resp).await;
    if !create_bid_status.is_success() {
        panic!("Failed to create bid: status: {}, body: {:?}", create_bid_status, create_bid_body);
    }
    let bid: Bid = serde_json::from_slice(&create_bid_body).unwrap();

    // Update the bid
    let req = test::TestRequest::put()
        .uri(&format!("/bid/{}", bid.id))
        .set_json(json!({
            "bid_amount": 150.0,
            "status": "accepted"
        }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let resp_status = resp.status();
    let resp_body = test::read_body(resp).await;
    if !resp_status.is_success() {
        panic!("Failed to update bid: status: {}, body: {:?}", resp_status, resp_body);
    }
    assert!(resp_status.is_success());
}

#[actix_rt::test]
async fn test_delete_bid() {
    let app = test::init_service(App::new().configure(bid::init)).await;

    // Create repair request first
    let repair_request_resp = test::TestRequest::post()
        .uri("/repair_request")
        .set_json(json!({
            "details": "Test repair request"
        }))
        .to_request();
    let repair_request_resp = test::call_service(&app, repair_request_resp).await;
    let repair_request_status = repair_request_resp.status();
    let repair_request_body = test::read_body(repair_request_resp).await;
    if !repair_request_status.is_success() {
        panic!("Failed to create repair request: status: {}, body: {:?}", repair_request_status, repair_request_body);
    }
    let repair_request: RepairRequest = serde_json::from_slice(&repair_request_body).unwrap();

    let repair_shop_resp = test::TestRequest::post()
        .uri("/repair_shop")
        .set_json(json!({
            "name": "Test Repair Shop"
        }))
        .to_request();
    let repair_shop_resp = test::call_service(&app, repair_shop_resp).await;
    let repair_shop_status = repair_shop_resp.status();
    let repair_shop_body = test::read_body(repair_shop_resp).await;
    if !repair_shop_status.is_success() {
        panic!("Failed to create repair shop: status: {}, body: {:?}", repair_shop_status, repair_shop_body);
    }
    let repair_shop: RepairShop = serde_json::from_slice(&repair_shop_body).unwrap();

    let create_bid_req = test::TestRequest::post()
        .uri("/bid")
        .set_json(json!({
            "repair_request_id": repair_request.id,
            "repair_shop_id": repair_shop.id,
            "bid_amount": 100.0,
            "status": "pending"
        }))
        .to_request();
    let create_bid_resp = test::call_service(&app, create_bid_req).await;
    let create_bid_status = create_bid_resp.status();
    let create_bid_body = test::read_body(create_bid_resp).await;
    if !create_bid_status.is_success() {
        panic!("Failed to create bid: status: {}, body: {:?}", create_bid_status, create_bid_body);
    }
    let bid: Bid = serde_json::from_slice(&create_bid_body).unwrap();

    // Delete the bid
    let req = test::TestRequest::delete()
        .uri(&format!("/bid/{}", bid.id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    let resp_status = resp.status();
    let resp_body = test::read_body(resp).await;
    if !resp_status.is_success() {
        panic!("Failed to delete bid: status: {}, body: {:?}", resp_status, resp_body);
    }
    assert!(resp_status.is_success());
}
