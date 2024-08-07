// Import the necessary modules and dependencies
use std::sync::Once;
use std::thread;
use std::time::Duration;
use backend::server::create_server;
use backend::utils::db::establish_connection;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::connection::SimpleConnection;
use serde::Deserialize;

static INIT: Once = Once::new();

pub fn initialize_test_server() {
    INIT.call_once(|| {
        dotenv::from_filename(".env.test").ok(); // Load test environment variables

        thread::spawn(|| {
            actix_rt::System::new().block_on(async {
                let server = create_server().await.expect("Failed to start server");
                server.await.expect("Server failed");
            });
        });

        thread::sleep(Duration::from_secs(1)); // Give the server some time to start
    });
}

pub fn setup_database() -> PgConnection {
    let mut conn = establish_connection();
    clean_up_database(&mut conn);
    conn
}

pub fn clean_up_database(conn: &mut PgConnection) {
    use backend::models::schema::{addresses, bids, orders, repair_requests, repair_shops, staff, status_pipeline, users};

    println!("Cleaning up database...");
    diesel::delete(status_pipeline::table).execute(conn).unwrap();
    diesel::delete(orders::table).execute(conn).unwrap();
    diesel::delete(bids::table).execute(conn).unwrap();
    diesel::delete(repair_requests::table).execute(conn).unwrap();
    diesel::delete(staff::table).execute(conn).unwrap();
    diesel::delete(addresses::table).execute(conn).unwrap();
    diesel::delete(repair_shops::table).execute(conn).unwrap();
    diesel::delete(users::table).execute(conn).unwrap();

    let reset_sequences_query = "
        DO $$ DECLARE
            r RECORD;
        BEGIN
            FOR r IN (SELECT c.relname FROM pg_class c WHERE c.relkind = 'S') LOOP
                EXECUTE 'ALTER SEQUENCE ' || r.relname || ' RESTART WITH 1;';
            END LOOP;
        END $$;
    ";

    conn.batch_execute(reset_sequences_query).unwrap();
    println!("Database cleanup completed.");
}

#[derive(Deserialize, Debug)]
struct RegisterResponse {
    user_id: String,
    token: String,
}

#[derive(Deserialize, Debug)]
struct RepairShopResponse {
    id: String,
}

#[derive(Deserialize, Debug)]
struct RepairRequestResponse {
    id: String,
}

#[derive(Deserialize, Debug)]
struct OrderResponse {
    id: String,
}

#[derive(Deserialize, Debug)]
struct BidResponse {
    id: String,
    repair_request_id: String,
    repair_shop_id: String,
    bid_amount: f64,
    status: String,
    created_at: String,
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use reqwest::Client;
    use serde_json::json;

    #[actix_rt::test]
    async fn test_workflow() {
        initialize_test_server();
        setup_database();

        let client = Client::new();

        // Register users
        let res = client.post("http://127.0.0.1:8080/auth/register")
            .json(&json!({ "email": "customer@example.com", "password": "password" }))
            .send().await.unwrap();
        println!("Register customer response: {:?}", res);
        assert!(res.status().is_success());
        let customer_response: RegisterResponse = res.json().await.unwrap();
        println!("Registered customer: {:?}", customer_response);
        let customer_token = customer_response.token;

        let res = client.post("http://127.0.0.1:8080/auth/register")
            .json(&json!({ "email": "staff1@example.com", "password": "password" }))
            .send().await.unwrap();
        println!("Register staff1 response: {:?}", res);
        assert!(res.status().is_success());
        let staff1_response: RegisterResponse = res.json().await.unwrap();
        println!("Registered staff1: {:?}", staff1_response);
        let staff1_token = staff1_response.token;

        let res = client.post("http://127.0.0.1:8080/auth/register")
            .json(&json!({ "email": "staff2@example.com", "password": "password" }))
            .send().await.unwrap();
        println!("Register staff2 response: {:?}", res);
        assert!(res.status().is_success());
        let staff2_response: RegisterResponse = res.json().await.unwrap();
        println!("Registered staff2: {:?}", staff2_response);
        let staff2_token = staff2_response.token;

        // Create repair shops
        let res = client.post("http://127.0.0.1:8080/repair_shop")
            .bearer_auth(&staff1_token)
            .json(&json!({ "name": "Shop 1" }))
            .send().await.unwrap();
        let status = res.status();
        let raw_body = res.text().await.unwrap();
        println!("Create repair shop 1 response status: {:?}", status);
        println!("Create repair shop 1 response body: {}", raw_body);
        let repair_shop1_id: RepairShopResponse = serde_json::from_str(&raw_body).unwrap();

        let res = client.post("http://127.0.0.1:8080/repair_shop")
            .bearer_auth(&staff2_token)
            .json(&json!({ "name": "Shop 2" }))
            .send().await.unwrap();
        let status = res.status();
        let raw_body = res.text().await.unwrap();
        println!("Create repair shop 2 response status: {:?}", status);
        println!("Create repair shop 2 response body: {}", raw_body);
        let repair_shop2_id: RepairShopResponse = serde_json::from_str(&raw_body).unwrap();

        // Add addresses for the repair shops
        let res = client.post("http://127.0.0.1:8080/address")
            .bearer_auth(&staff1_token)
            .json(&json!({
                "repair_shop_id": repair_shop1_id.id,
                "street": "123 Main St",
                "city": "City",
                "state": "State",
                "zip": "12345",
                "country": "Country"
            }))
            .send().await.unwrap();
        let status = res.status();
        let raw_body = res.text().await.unwrap();
        println!("Add address response for Shop 1 status: {:?}", status);
        println!("Add address response for Shop 1 body: {}", raw_body);
        assert!(status.is_success());

        let res = client.post("http://127.0.0.1:8080/address")
            .bearer_auth(&staff2_token)
            .json(&json!({
                "repair_shop_id": repair_shop2_id.id,
                "street": "456 Elm St",
                "city": "City",
                "state": "State",
                "zip": "67890",
                "country": "Country"
            }))
            .send().await.unwrap();
        let status = res.status();
        let raw_body = res.text().await.unwrap();
        println!("Add address response for Shop 2 status: {:?}", status);
        println!("Add address response for Shop 2 body: {}", raw_body);
        assert!(status.is_success());

        // Customer submits a repair request
        let res = client.post("http://127.0.0.1:8080/repair_request")
            .bearer_auth(&customer_token)
            .json(&json!({
                "customer_id": customer_response.user_id,
                "description": "Fix my shoes"
            }))
            .send().await.unwrap();
        let status = res.status();
        let raw_body = res.text().await.unwrap();
        println!("Repair request response status: {:?}", status);
        println!("Repair request response body: {}", raw_body);
        let repair_request_id: RepairRequestResponse = serde_json::from_str(&raw_body).unwrap();

        // Staff members submit bids
        let res = client.post("http://127.0.0.1:8080/bid")
            .bearer_auth(&staff1_token)
            .json(&json!({
                "repair_request_id": repair_request_id.id,
                "repair_shop_id": repair_shop1_id.id,
                "status": "pending",
                "bid_amount": 50.0
            }))
            .send().await.unwrap();
        let status = res.status();
        let raw_body = res.text().await.unwrap();
        println!("Submit bid response for staff1 status: {:?}", status);
        println!("Submit bid response for staff1 body: {}", raw_body);
        let bid1_id: BidResponse = serde_json::from_str(&raw_body).unwrap();
        println!("Bid 1 ID: {}", bid1_id.id);
        println!("Bid 1 Repair Request ID: {}", bid1_id.repair_request_id);
        println!("Bid 1 Repair Shop ID: {}", bid1_id.repair_shop_id);
        println!("Bid 1 Amount: {}", bid1_id.bid_amount);
        println!("Bid 1 Status: {}", bid1_id.status);
        println!("Bid 1 Created At: {}", bid1_id.created_at);

        let res = client.post("http://127.0.0.1:8080/bid")
            .bearer_auth(&staff2_token)
            .json(&json!({
                "repair_request_id": repair_request_id.id,
                "repair_shop_id": repair_shop2_id.id,
                "status": "pending",
                "bid_amount": 45.0
            }))
            .send().await.unwrap();
        let status = res.status();
        let raw_body = res.text().await.unwrap();
        println!("Submit bid response for staff2 status: {:?}", status);
        println!("Submit bid response for staff2 body: {}", raw_body);
        let bid2_id: BidResponse = serde_json::from_str(&raw_body).unwrap();
        println!("Bid 2 ID: {}", bid2_id.id);
        println!("Bid 2 Repair Request ID: {}", bid2_id.repair_request_id);
        println!("Bid 2 Repair Shop ID: {}", bid2_id.repair_shop_id);
        println!("Bid 2 Amount: {}", bid2_id.bid_amount);
        println!("Bid 2 Status: {}", bid2_id.status);
        println!("Bid 2 Created At: {}", bid2_id.created_at);

        // Customer accepts the lowest bid and creates an order
        let res = client.post("http://127.0.0.1:8080/order")
            .bearer_auth(&customer_token)
            .json(&json!({
                "repair_request_id": repair_request_id.id,
                "repair_shop_id": repair_shop2_id.id,
                "status": "accepted"
            }))
            .send().await.unwrap();
        let status = res.status();
        let raw_body = res.text().await.unwrap();
        println!("Create order response status: {:?}", status);
        println!("Create order response body: {}", raw_body);
        assert!(status.is_success());

        // Staff updates the order status using PATCH
        let order_id: OrderResponse = serde_json::from_str(&raw_body).unwrap();
        let res = client.patch(&format!("http://127.0.0.1:8080/order/{}", order_id.id))
            .bearer_auth(&staff2_token)
            .json(&json!({ "status": "in_progress" }))
            .send().await.unwrap();
        let status = res.status();
        let raw_body = res.text().await.unwrap();
        println!("Update order status response status: {:?}", status);
        println!("Update order status response body: {}", raw_body);
        assert!(status.is_success());

        // Add status pipeline entry
        let res = client.post("http://127.0.0.1:8080/status_pipeline")
            .bearer_auth(&staff2_token)
            .json(&json!({
                "order_id": order_id.id,
                "status": "in_progress"
            }))
            .send().await.unwrap();
        let status = res.status();
        let raw_body = res.text().await.unwrap();
        println!("Add status pipeline response status: {:?}", status);
        println!("Add status pipeline response body: {}", raw_body);
        assert!(status.is_success());
    }
}
