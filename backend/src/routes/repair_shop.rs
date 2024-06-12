use actix_web::web;
use log::info;
use crate::handlers::repair_shop::{create_repair_shop, get_repair_shop, update_repair_shop, delete_repair_shop};

pub fn init(cfg: &mut web::ServiceConfig) {
    info!("Configuring repair shop routes");
    cfg.service(
        web::resource("/repair_shop/{id}")
            .route(web::get().to(get_repair_shop))
            .route(web::put().to(update_repair_shop))
            .route(web::delete().to(delete_repair_shop)),
    )
    .service(
        web::resource("/repair_shop")
            .route(web::post().to(create_repair_shop)),
    );
}

