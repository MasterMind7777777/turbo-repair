use actix_web::web;
use crate::handlers::auth::login;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/login")
            .route(web::post().to(login))
    );
}

