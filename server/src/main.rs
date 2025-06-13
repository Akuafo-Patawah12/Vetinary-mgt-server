use actix_web::{get,web,http, App, HttpServer, Responder, HttpResponse};

use actix_cors::Cors;

mod services;
mod models;
mod routes;

use services::db::Database;

use crate::routes::{booking_route::{cancel_bookings, get_bookings, create_booking}, owner_route::{create_owner} , dog_route::create_dog};


#[get("/hello")]
async fn call() -> impl Responder {
    HttpResponse::Ok().body("parcch!")
}

#[actix_web::main]
#[doc = " Main actix-web server function that initializes the server and sets up routes."]
async fn main() -> std::io::Result<()> {

    println!(" Server running on port 5000");
    
    let db: Database = Database::init().await;
    let db_data = web::Data::new(db);
    HttpServer::new( move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin_fn(|origin,_req_head|{
                origin.as_bytes().ends_with(b".localhost:3000")
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(db_data.clone())
            .service(call) // register route
            .service(create_dog)
            .service(create_owner)
            .service(create_booking)
            .service(get_bookings)
            .service(cancel_bookings)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}