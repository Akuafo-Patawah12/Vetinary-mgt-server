use actix_web::{ post ,web::{Data, Json}, HttpResponse};
use crate::services::db::Database;
use crate::models::{ owner_model::{Owner,OwnerRequest}};




#[post("/dog")]
pub async fn create_owner(db: Data<Database>, request: Json<OwnerRequest>) -> HttpResponse {
   match db
       .create_owner(
          Owner::try_from( OwnerRequest {
              _id: request._id.clone(),
              name: request.name.clone(),
              email: request.email.clone(),
              address: request.address.clone(),
          })
          .expect("Error converting DogRequest to Dog")

       ).await {
       Ok(owner) => HttpResponse::Ok().json(owner),
         Err(err) => {
              eprintln!("Error creating owner: {}", err);
              HttpResponse::InternalServerError().body(err.to_string())
         }
        }
}