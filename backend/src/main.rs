use dotenv::dotenv;
use crate::server::create_server;

mod handlers;
mod models;
mod routes;
mod services;
mod utils;
mod middleware;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load production environment variables
    let server = create_server().await?;
    server.await
}

