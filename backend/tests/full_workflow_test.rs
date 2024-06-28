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

#[derive(Deserialize)]
struct RegisterResponse {
    id: String,
}

#[derive(Deserialize)]
struct RepairShopResponse {
    id: String,
}

#[derive(Deserialize)]
struct RepairRequestResponse {
    id: String,
}

#[derive(Deserialize)]
struct OrderResponse {
    id: String,
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
        println!("Registered customer ID: {}", customer_response.id);

        let res = client.post("http://127.0.0.1:8080/auth/register")
            .json(&json!({ "email": "staff1@example.com", "password": "password" }))
            .send().await.unwrap();
        println!("Register staff1 response: {:?}", res);
        assert!(res.status().is_success());
        let staff1_response: RegisterResponse = res.json().await.unwrap();
        println!("Registered staff1 ID: {}", staff1_response.id);

        let res = client.post("http://127.0.0.1:8080/auth/register")
            .json(&json!({ "email": "staff2@example.com", "password": "password" }))
            .send().await.unwrap();
        println!("Register staff2 response: {:?}", res);
        assert!(res.status().is_success());
        let staff2_response: RegisterResponse = res.json().await.unwrap();
        println!("Registered staff2 ID: {}", staff2_response.id);

        // Log in users and get tokens
        let res = client.post("http://127.0.0.1:8080/auth/login")
            .json(&json!({ "email": "customer@example.com", "password": "password" }))
            .send().await.unwrap();
        println!("Login customer response: {:?}", res);
        let customer_token: String = res.text().await.unwrap().replace('"', "");
        println!("Customer token: {}", customer_token);

        let res = client.post("http://127.0.0.1:8080/auth/login")
            .json(&json!({ "email": "staff1@example.com", "password": "password" }))
            .send().await.unwrap();
        println!("Login staff1 response: {:?}", res);
        let staff1_token: String = res.text().await.unwrap().replace('"', "");
        println!("Staff1 token: {}", staff1_token);

        let res = client.post("http://127.0.0.1:8080/auth/login")
            .json(&json!({ "email": "staff2@example.com", "password": "password" }))
            .send().await.unwrap();
        println!("Login staff2 response: {:?}", res);
        let staff2_token: String = res.text().await.unwrap().replace('"', "");
        println!("Staff2 token: {}", staff2_token);

        // Create repair shops
        let res = client.post("http://127.0.0.1:8080/repair_shop")
            .bearer_auth(&staff1_token)
            .json(&json!({ "name": "Shop 1" }))
            .send().await.unwrap();
        println!("Create repair shop 1 response: {:?}", res);
        let repair_shop1_id: RepairShopResponse = res.json().await.unwrap();

        let res = client.post("http://127.0.0.1:8080/repair_shop")
            .bearer_auth(&staff2_token)
            .json(&json!({ "name": "Shop 2" }))
            .send().await.unwrap();
        println!("Create repair shop 2 response: {:?}", res);
        let repair_shop2_id: RepairShopResponse = res.json().await.unwrap();

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
            .send().await;
        println!("Add address response for Shop 1: {:?}", res);
        assert!(res.unwrap().status().is_success());

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
            .send().await;
        println!("Add address response for Shop 2: {:?}", res);
        assert!(res.unwrap().status().is_success());

        // Customer submits a repair request
        let res = client.post("http://127.0.0.1:8080/repair_request")
            .bearer_auth(&customer_token)
            .json(&json!({
                "customer_id": customer_response.id,
                "description": "Fix my shoes"
            }))
            .send().await;
        println!("Repair request response: {:?}", res);
        let repair_request_id: RepairRequestResponse = res.unwrap().json().await.unwrap();

        // Staff members submit bids
        let res = client.post("http://127.0.0.1:8080/bid")
            .bearer_auth(&staff1_token)
            .json(&json!({
                "repair_request_id": repair_request_id.id,
                "repair_shop_id": repair_shop1_id.id,
                "bid_amount": 50.0
            }))
            .send().await.unwrap();
        println!("Submit bid response for staff1: {:?}", res);
        let _bid1_id: RegisterResponse = res.json().await.unwrap();

        let res = client.post("http://127.0.0.1:8080/bid")
            .bearer_auth(&staff2_token)
            .json(&json!({
                "repair_request_id": repair_request_id.id,
                "repair_shop_id": repair_shop2_id.id,
                "bid_amount": 45.0
            }))
            .send().await.unwrap();
        println!("Submit bid response for staff2: {:?}", res);
        let _bid2_id: RegisterResponse = res.json().await.unwrap();

        // Customer accepts the lowest bid and creates an order
        let res = client.post("http://127.0.0.1:8080/orders")
            .bearer_auth(&customer_token)
            .json(&json!({
                "repair_request_id": repair_request_id.id,
                "repair_shop_id": repair_shop2_id.id,
                "status": "accepted"
            }))
            .send().await.unwrap();
        println!("Create order response: {:?}", res);
        assert!(res.status().is_success());

        // Staff updates the order status
        let order_id: OrderResponse = res.json().await.unwrap();
        let res = client.put(&format!("http://127.0.0.1:8080/orders/{}", order_id.id))
            .bearer_auth(&staff2_token)
            .json(&json!({ "status": "in_progress" }))
            .send().await.unwrap();
        println!("Update order status response: {:?}", res);
        assert!(res.status().is_success());

        // Add status pipeline entry
        let res = client.post("http://127.0.0.1:8080/status_pipeline")
            .bearer_auth(&staff2_token)
            .json(&json!({
                "order_id": order_id.id,
                "status": "in_progress"
            }))
            .send().await.unwrap();
        println!("Add status pipeline response: {:?}", res);
        assert!(res.status().is_success());
    }
}
