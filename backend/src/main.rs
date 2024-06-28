use dotenv::dotenv;
use env_logger::Env;
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
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let server = create_server().await?;
    server.await
}

