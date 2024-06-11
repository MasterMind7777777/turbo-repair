use actix_web::web;
use log::info;
use crate::handlers::user::{get_user_details, update_user_profile};

pub fn init(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes");
    cfg.service(
        web::resource("/user/{id}")
            .route(web::get().to(get_user_details))
            .route(web::put().to(update_user_profile)),
    );
}
