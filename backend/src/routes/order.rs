use actix_web::web;
use log::info;
use crate::handlers::order::{create_order, delete_order, get_order, get_orders, patch_order_status, update_order};

pub fn init(cfg: &mut web::ServiceConfig) {
    info!("Configuring order routes");
    cfg.service(
        web::resource("/order/{id}")
            .route(web::get().to(get_order))
            .route(web::put().to(update_order))
            .route(web::patch().to(patch_order_status))
            .route(web::delete().to(delete_order)),
    )
    .service(
        web::resource("/order")
            .route(web::post().to(create_order))
            .route(web::get().to(get_orders)),
    );
}

