mod user_routes;
mod auth_routes;
mod repair_shop_routes;
mod repair_request_routes;
mod bid_routes;
mod order_routes;
mod helpers;
use dotenv::dotenv;

fn main() {
    dotenv().ok();  // Load environment variables from .env file
    // This is needed for Cargo to compile 
    // and run the tests in this directory
}

