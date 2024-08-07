use actix_cors::Cors;
use actix_files as fs;
use actix_web::http;
use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use actix_web::dev::Server;
use env_logger::Env;
use log::info;
use serde_json::json;
use crate::middleware::auth::Auth;
use crate::routes;


pub async fn create_server() -> std::io::Result<Server> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    info!("Starting server at http://127.0.0.1:8080");

    // Create JsonConfig with custom error handler
    let json_config = web::JsonConfig::default().error_handler(|err, _req| {
        let err_str = format!("Invalid input: {}", err);
        actix_web::error::InternalError::from_response(
            err, HttpResponse::BadRequest().json(json!({ "error": err_str.clone() }))
        ).into()
    });

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // or specify allowed origins using .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .app_data(json_config.clone()) // Apply custom JsonConfig to the App
            .wrap(cors) // Apply Cors middleware
            .wrap(Logger::default()) // Use Actix's built-in Logger middleware
            .service(
                web::scope("/api/auth")
                    .configure(routes::auth)  // Public routes under /auth
            )
            .service(
                web::scope("/api") // All other routes
                    .wrap(Auth)  // Apply Auth middleware to these routes
                    .configure(routes::init)  // Protected routes
            )
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run();

    Ok(server)
}


