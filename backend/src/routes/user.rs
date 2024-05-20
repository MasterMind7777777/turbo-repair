use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use log::info;
use crate::handlers::user::{get_user_details, register_user, update_user_profile};
use crate::middleware::public::Public;
use crate::models::user::UserInput;

async fn register(req: HttpRequest, payload: web::Json<UserInput>) -> HttpResponse {
    req.extensions_mut().insert(Public);
    register_user(payload).await
}

pub fn init(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes");
    cfg.service(
        web::resource("/register")
            .route(web::post().to(register)),
    );
    cfg.service(
        web::resource("/user/{id}")
            .route(web::get().to(get_user_details))
            .route(web::put().to(update_user_profile)),
    );
}
