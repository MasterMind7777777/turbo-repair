use actix_web::web;
use crate::handlers::status_pipeline::{create_status, delete_status, get_status, get_statuses, update_status};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/status_pipeline")
            .route(web::get().to(get_statuses))
            .route(web::post().to(create_status)),
    );
    cfg.service(
        web::resource("/status_pipeline/{id}")
            .route(web::get().to(get_status))
            .route(web::put().to(update_status))
            .route(web::delete().to(delete_status)),
    );
}

