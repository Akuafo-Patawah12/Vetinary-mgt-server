use actix_web::{get,web, App, HttpServer, Responder, HttpResponse};

mod services;
mod models;

use services::db::Database;


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
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}