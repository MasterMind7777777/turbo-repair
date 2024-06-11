use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use serde_json::json;
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

    // Create JsonConfig with custom error handler
    let json_config = web::JsonConfig::default().error_handler(|err, _req| {
        let err_str = format!("Invalid input: {}", err);
        actix_web::error::InternalError::from_response(
            err, HttpResponse::BadRequest().json(json!({ "error": err_str.clone() }))
        ).into()
    });

    HttpServer::new(move || {
        App::new()
            .app_data(json_config.clone()) // Apply custom JsonConfig to the App
            .wrap(Logger::default()) // Use Actix's built-in Logger middleware
            .service(
                web::scope("/auth")
                    .configure(routes::auth)  // Public routes under /auth
            )
            .service(
                web::scope("") // All other routes
                    .wrap(Auth)  // Apply Auth middleware to these routes
                    .configure(routes::init)  // Protected routes
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

