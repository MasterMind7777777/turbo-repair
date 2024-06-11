pub mod user;
pub mod auth;

use actix_web::web;

pub fn auth(cfg: &mut web::ServiceConfig) {
    auth::init(cfg);
}

pub fn init(cfg: &mut web::ServiceConfig) {
    user::init(cfg);
}

