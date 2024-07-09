use actix_web::web;
use log::info;
use crate::handlers::staff_task::{
    create_staff_task, delete_staff_task, get_staff_task, get_staff_tasks, link_staff_to_task, patch_staff_task, update_staff_task
};

pub fn init(cfg: &mut web::ServiceConfig) {
    info!("Configuring staff task routes");
    cfg.service(
        web::resource("/staff_task")
            .route(web::post().to(create_staff_task))
            .route(web::get().to(get_staff_tasks)),
    )
    .service(
        web::resource("/staff_task/{id}")
            .route(web::get().to(get_staff_task))
            .route(web::put().to(update_staff_task))
            .route(web::patch().to(patch_staff_task))
            .route(web::delete().to(delete_staff_task)),
    )
    .service(
        web::resource("/staff_task/link")
            .route(web::post().to(link_staff_to_task)),
    );
}

