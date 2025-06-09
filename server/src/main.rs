use actix_web::{get,web, App, HttpServer, Responder, HttpResponse};

mod services;
mod models;
mod routes;

use services::db::Database;

use crate::routes::{booking_route::{cancel_bookings, create_dog, get_bookings, create_bookings}, owner_route::create_owner};


#[get("/hello")]
async fn call() -> impl Responder {
    HttpResponse::Ok().body("parcch!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!(" Server running on port 5000");
    
    let db = Database::init().await;
    let db_data = web::Data::new(db);
    HttpServer::new( move || {
        App::new()
            .app_data(db_data.clone())
            .service(call) // register route
            .service(create_dog)
            .service(create_owner)
            .service(create_bookings)
            .service(get_bookings)
            .service(cancel_bookings(db, path))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}