use crate::handlers::bid::{
    accept_bid, create_bid, delete_bid, get_bid, get_bids_for_request, update_bid,
};
use actix_web::web;
use log::info;

pub fn init(cfg: &mut web::ServiceConfig) {
    info!("Configuring bid routes");
    cfg.service(
        web::resource("/bid/{id}")
            .route(web::get().to(get_bid))
            .route(web::put().to(update_bid))
            .route(web::delete().to(delete_bid)),
    )
    .service(web::resource("/bid").route(web::post().to(create_bid)))
    .service(web::resource("/bids/for_request").route(web::get().to(get_bids_for_request)))
    .service(web::resource("/bid/{id}/accept").route(web::post().to(accept_bid)));
}
