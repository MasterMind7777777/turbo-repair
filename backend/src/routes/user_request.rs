use actix_web::web;
use log::info;
use crate::handlers::user_request::{create_user_request, delete_user_request, get_user_request, get_user_requests, update_user_request, patch_user_request};

pub fn init(cfg: &mut web::ServiceConfig) {
    info!("Configuring user request routes");
    cfg.service(
        web::resource("/user_request")
            .route(web::post().to(create_user_request))
            .route(web::get().to(get_user_requests)),
    )
    .service(
        web::resource("/user_request/{id}")
            .route(web::get().to(get_user_request))
            .route(web::put().to(update_user_request))
            .route(web::patch().to(patch_user_request))
            .route(web::delete().to(delete_user_request)),
    );
}

