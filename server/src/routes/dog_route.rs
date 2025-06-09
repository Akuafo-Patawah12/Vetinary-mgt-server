use actix_web::{ post ,web::{Data, Json}, HttpResponse};
use mongodb::Database;

use crate::models::dog_model::{Dog,DogRequest};




#[post("/dog")]
pub async fn create_dog(db: Data<Database>, request: Json<DogRequest>) -> HttpResponse {
   match db
       .create_dog(
          Dog::try_from(DogRequest {
              owner: request.owner,
              name: request.name.clone(),
              age: request.age,
              breed: request.breed.clone(),
          })
          .expect("Error converting DogRequest to Dog")

       ).await {
       Ok(dog) => HttpResponse::Ok().json(dog),
         Err(err) => {
              eprintln!("Error creating dog: {}", e);
              HttpResponse::InternalServerError().body(err.to_string())
         }
        }
}