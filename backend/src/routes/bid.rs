use actix_web::web;
use log::info;
use crate::handlers::bid::{create_bid, get_bid, update_bid, delete_bid};

pub fn init(cfg: &mut web::ServiceConfig) {
    info!("Configuring bid routes");
    cfg.service(
        web::resource("/bid/{id}")
            .route(web::get().to(get_bid))
            .route(web::put().to(update_bid))
            .route(web::delete().to(delete_bid)),
    )
    .service(
        web::resource("/bid")
            .route(web::post().to(create_bid)),
    );
}

