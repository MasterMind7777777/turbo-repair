use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use crate::middleware::auth::Auth;

mod handlers;
mod models;
mod routes;
mod services;
mod utils;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Auth)
            .configure(routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

