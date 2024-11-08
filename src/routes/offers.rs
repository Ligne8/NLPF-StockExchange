use actix_web::web;

use crate::controllers::offers::{create_lot_offer, get_all_lots};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_lots);
    cfg.service(create_lot_offer);
    // Ajoutez d'autres routes ici
}