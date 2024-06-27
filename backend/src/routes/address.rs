use actix_web::web;
use crate::handlers::address::{create_address, get_address, update_address, delete_address};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/address/{id}")
            .route(web::get().to(get_address))
            .route(web::put().to(update_address))
            .route(web::delete().to(delete_address)),
    )
    .service(
        web::resource("/address")
            .route(web::post().to(create_address)),
    );
}

