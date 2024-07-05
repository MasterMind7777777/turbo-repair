use actix_web::web;
use crate::handlers::address::{
    create_address, delete_address, get_address, partially_update_address, update_address, get_address_by_shop_id
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/address/{id}")
            .route(web::get().to(get_address))
            .route(web::put().to(update_address))
            .route(web::patch().to(partially_update_address))
            .route(web::delete().to(delete_address)),
    )
    .service(
        web::resource("/address")
            .route(web::post().to(create_address)),
    )
    .service(
        web::resource("/address/by_shop/{shop_id}")
            .route(web::get().to(get_address_by_shop_id)),
    );
}

