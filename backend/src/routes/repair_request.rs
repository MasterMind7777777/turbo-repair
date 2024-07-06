use crate::handlers::repair_request::{
    create_repair_request, delete_repair_request, get_repair_request, list_available_requests,
    list_user_repair_requests, update_repair_request,
};
use actix_web::web;
use log::info;

pub fn init(cfg: &mut web::ServiceConfig) {
    info!("Configuring repair request routes");
    cfg.service(
        web::resource("/repair_request/{id}")
            .route(web::get().to(get_repair_request))
            .route(web::put().to(update_repair_request))
            .route(web::delete().to(delete_repair_request)),
    )
    .service(
        web::resource("/repair_request")
            .route(web::post().to(create_repair_request))
            .route(web::get().to(list_user_repair_requests)),
    )
    .service(web::resource("/available_requests").route(web::get().to(list_available_requests)));
}
