use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/hello")]
async fn call() -> impl Responder {
    HttpResponse::Ok().body("parcch!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!(" Server running on port 5000");
    
    let db = database::init().await;
    let db_data = web::Data::new(db);
    HttpServer::new(|| {
        App::new()
            .service(call) // register route
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}