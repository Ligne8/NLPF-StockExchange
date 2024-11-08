use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use std::sync::Arc;

mod controllers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL doit être défini");
    let client_options = ClientOptions::parse(&database_url)
        .await
        .expect("Erreur lors de l'analyse de l'URI MongoDB");
    let client = Client::with_options(client_options).expect("Erreur lors de la création du client MongoDB");
    let db = client.database("StockExchange");
    let db_data = web::Data::new(Arc::new(db));
    println!("Serveur démarré sur http://localhost:5151");
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .configure(routes::init)
    })
    .bind("127.0.0.1:5151")?
    .run()
    .await
    
}