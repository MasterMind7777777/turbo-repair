use actix_web::web;
use crate::handlers::staff::{create_staff, delete_staff, get_staff, get_staffs, update_staff};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/staff")
            .route(web::get().to(get_staffs))
            .route(web::post().to(create_staff)),
    );
    cfg.service(
        web::resource("/staff/{id}")
            .route(web::get().to(get_staff))
            .route(web::put().to(update_staff))
            .route(web::delete().to(delete_staff)),
    );
}

