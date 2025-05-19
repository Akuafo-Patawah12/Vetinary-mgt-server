use actix_web::{get, patch, post, App, HttpServer, Responder, HttpResponse};

#[get("/hello")]
async fn call() -> impl Responder {
    HttpResponse::Ok().body("parcch!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!(" Server running on port 5000");
    
    HttpServer::new(|| {
        App::new()
            .service(call) // register route
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
