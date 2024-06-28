pub mod user;
pub mod auth;
pub mod repair_shop;
pub mod repair_request;
pub mod bid;
pub mod order;
pub mod address;
pub mod staff;
pub mod status_pipeline;

use actix_web::web;



pub fn auth(cfg: &mut web::ServiceConfig) {
    auth::init(cfg);
}

pub fn init(cfg: &mut web::ServiceConfig) {
    user::init(cfg);
    repair_shop::init(cfg);
    repair_request::init(cfg);
    bid::init(cfg);
    order::init(cfg);
    address::init(cfg);
    staff::init(cfg);
    status_pipeline::init(cfg);
}

