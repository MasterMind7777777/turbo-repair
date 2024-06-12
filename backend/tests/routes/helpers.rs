use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;
use backend::models::{user::User, repair_request::RepairRequest, repair_shop::RepairShop, bid::Bid, order::Order, address::Address, staff::Staff, status_pipeline::StatusPipeline, schema::{addresses, bids, orders, repair_requests, repair_shops, staff, status_pipeline, users}};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new("postgres://test_user:test_password@localhost/test_database");
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn create_test_database() {
    let connection = PgConnection::establish("postgres://test_user:test_password@localhost/postgres")
        .expect("Failed to connect to PostgreSQL");

    connection
        .batch_execute("CREATE DATABASE test_database")
        .expect("Failed to create test database");

    // Run migrations on the test database
    let test_connection = PgConnection::establish("postgres://test_user:test_password@localhost/test_database")
        .expect("Failed to connect to test database");
    diesel_migrations::run_pending_migrations(&test_connection)
        .expect("Failed to run migrations on test database");
}

pub fn drop_test_database() {
    let connection = PgConnection::establish("postgres://test_user:test_password@localhost/postgres")
        .expect("Failed to connect to PostgreSQL");

    // Terminate all connections to the database
    connection
        .batch_execute(
            "
            SELECT pg_terminate_backend(pid)
            FROM pg_stat_activity
            WHERE datname = 'test_database' AND pid <> pg_backend_pid();
            DROP DATABASE IF EXISTS test_database;
            ",
        )
        .expect("Failed to drop test database");
}

pub fn create_user(conn: &mut PgConnection) -> User {
    let new_user = User {
        id: Uuid::new_v4(),
        email: "test@example.com".to_string(),
        password: "password".to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn create_repair_request(conn: &mut PgConnection, customer_id: Uuid) -> RepairRequest {
    let new_request = RepairRequest {
        id: Uuid::new_v4(),
        customer_id,
        description: "Test repair request".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(repair_requests::table)
        .values(&new_request)
        .get_result(conn)
        .expect("Error saving new repair request")
}

pub fn create_repair_shop(conn: &mut PgConnection) -> RepairShop {
    let new_shop = RepairShop {
        id: Uuid::new_v4(),
        name: "Test Repair Shop".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(repair_shops::table)
        .values(&new_shop)
        .get_result(conn)
        .expect("Error saving new repair shop")
}

pub fn create_address(conn: &mut PgConnection, repair_shop_id: Uuid) -> Address {
    let new_address = Address {
        id: Uuid::new_v4(),
        repair_shop_id: Some(repair_shop_id),
        street: "123 Main St".to_string(),
        city: "Test City".to_string(),
        state: "Test State".to_string(),
        zip: "12345".to_string(),
        country: "Test Country".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(addresses::table)
        .values(&new_address)
        .get_result(conn)
        .expect("Error saving new address")
}

pub fn create_bid(conn: &mut PgConnection, repair_request_id: Uuid, repair_shop_id: Uuid) -> Bid {
    let new_bid = Bid {
        id: Uuid::new_v4(),
        repair_request_id,
        repair_shop_id,
        bid_amount: 100.0,
        status: "pending".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(bids::table)
        .values(&new_bid)
        .get_result(conn)
        .expect("Error saving new bid")
}

pub fn create_order(conn: &mut PgConnection, repair_request_id: Uuid, repair_shop_id: Uuid) -> Order {
    let new_order = Order {
        id: Uuid::new_v4(),
        repair_request_id,
        repair_shop_id,
        status: "created".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(conn)
        .expect("Error saving new order")
}

pub fn create_staff(conn: &mut PgConnection, user_id: Uuid, repair_shop_id: Uuid) -> Staff {
    let new_staff = Staff {
        id: Uuid::new_v4(),
        user_id: Some(user_id),
        repair_shop_id: Some(repair_shop_id),
        role: "manager".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(staff::table)
        .values(&new_staff)
        .get_result(conn)
        .expect("Error saving new staff")
}

pub fn create_status_pipeline(conn: &mut PgConnection, order_id: Uuid) -> StatusPipeline {
    let new_status_pipeline = StatusPipeline {
        id: Uuid::new_v4(),
        order_id: Some(order_id),
        status: "created".to_string(),
        timestamp: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(status_pipeline::table)
        .values(&new_status_pipeline)
        .get_result(conn)
        .expect("Error saving new status pipeline")
}
