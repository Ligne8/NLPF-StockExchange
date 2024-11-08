use actix_web::{get, post, web, HttpResponse, Responder};
use futures::TryStreamExt;
use mongodb::Database;
use std::sync::Arc;

use crate::models::lot::Lot;

#[get("/offers/lots")]
async fn get_all_lots(db: web::Data<Arc<Database>>) -> impl Responder {
    println!("Requête reçue pour obtenir tous les lots");
    let collection = db.collection::<Lot>("lots");
    match collection.find(None, None).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Lot>>().await {
                Ok(results) => {
                    println!("Nombre de lots trouvés : {}", results.len());
                    HttpResponse::Ok().json(results)
                },
                Err(e) => {
                    println!("Erreur lors de la collecte des lots : {:?}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(e) => {
            println!("Erreur lors de la recherche des lots : {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/offers/lots")]
async fn create_lot_offer(db: web::Data<Arc<Database>>, new_lot: web::Json<Lot>) -> impl Responder {
    println!("Requête reçue pour créer un nouveau lot");
    let collection = db.collection::<Lot>("lots");
    match collection.insert_one(new_lot.into_inner(), None).await {
        Ok(inserted) => {
            println!("Lot créé avec succès : {:?}", inserted.inserted_id);
            HttpResponse::Created().json(inserted.inserted_id)
        },
        Err(e) => {
            println!("Erreur lors de la création du lot : {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}