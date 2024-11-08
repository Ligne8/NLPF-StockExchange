pub mod offers;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.configure(offers::init);
}