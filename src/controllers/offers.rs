use actix_web::{get, post, web, HttpResponse, Responder};
use futures::TryStreamExt;
use mongodb::Database;
use std::sync::Arc;

use crate::models::lot::Lot;

#[get("/offers/lots")]
async fn get_all_lots(db: web::Data<Arc<Database>>) -> impl Responder {
    let collection = db.collection::<Lot>("lots");
    let cursor = collection.find(None, None).await.unwrap();
    let results: Vec<Lot> = cursor.try_collect().await.unwrap();
    HttpResponse::Ok().json(results)
}

#[post("/offers/lots")]
async fn create_lot_offer(db: web::Data<Arc<Database>>, new_lot: web::Json<Lot>) -> impl Responder {
    let collection = db.collection::<Lot>("lots");
    let result = collection.insert_one(new_lot.into_inner(), None).await;
    match result {
        Ok(inserted) => HttpResponse::Created().json(inserted.inserted_id),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
