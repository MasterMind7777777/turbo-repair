use diesel::connection::SimpleConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use once_cell::sync::Lazy;
use uuid::Uuid;
use backend::models::{
    user::User,
    repair_request::RepairRequest,
    repair_shop::RepairShop,
    bid::Bid,
    order::Order,
    address::Address,
    staff::Staff,
    status_pipeline::StatusPipeline,
    schema::{addresses, bids, orders, repair_requests, repair_shops, staff, status_pipeline, users},
};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub static INIT: Lazy<DbPool> = Lazy::new(|| {
    dotenv::from_filename(".env.test").ok(); // Load test environment variables
    create_test_database();
    establish_connection()
});

pub fn establish_connection() -> DbPool {
    dotenv::from_filename(".env.test").ok(); // Load test environment variables
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Establishing connection to database: {}", database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(15)  // Example: setting max connections to 15
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn create_test_database() {
    dotenv::from_filename(".env.test").ok(); // Load test environment variables
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    {
        // Establish and then immediately drop a connection to ensure any previous connections are closed
        let _connection = PgConnection::establish(&database_url)
            .expect("Failed to connect to PostgreSQL");

        // Connection goes out of scope here, dropping it and ensuring it's closed
    }

    // Connect to the default database
    let mut connection = PgConnection::establish("postgres://test_user:test_password@localhost:5433/postgres")
        .expect("Failed to connect to PostgreSQL");

    // Drop and create the test database
    connection
        .batch_execute("DROP DATABASE IF EXISTS test_database WITH (FORCE);")
        .expect("Failed to drop test database");

    connection
        .batch_execute("CREATE DATABASE test_database")
        .expect("Failed to create test database");

    // Run migrations on the test database
    let test_database_url = "postgres://test_user:test_password@localhost:5433/test_database";
    let mut test_connection = PgConnection::establish(test_database_url)
        .expect("Failed to connect to test database");
    test_connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations on test database");
}

pub fn clean_up_database(conn: &mut PgConnection) {
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

pub fn create_user(conn: &mut PgConnection) -> User {
    let unique_email = format!("test+{}@example.com", Uuid::new_v4());
    let new_user = User {
        id: Uuid::new_v4(),
        email: unique_email,
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
        repair_shop_id,
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
        user_id,
        repair_shop_id,
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
        order_id,
        status: "created".to_string(),
        timestamp: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(status_pipeline::table)
        .values(&new_status_pipeline)
        .get_result(conn)
        .expect("Error saving new status pipeline")
}

