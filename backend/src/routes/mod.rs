pub mod user;
pub mod auth;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    user::init(cfg);
    auth::init(cfg);
}

