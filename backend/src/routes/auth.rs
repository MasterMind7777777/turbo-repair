use actix_web::web;
use crate::handlers::auth::{login, register_user};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/register")
            .route(web::post().to(register_user)),
    );
    cfg.service(
        web::resource("/login")
            .route(web::post().to(login))
    );
}

