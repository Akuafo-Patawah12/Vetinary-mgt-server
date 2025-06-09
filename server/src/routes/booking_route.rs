use actix_web::{ get, post, put, web::{Data, Json}, HttpResponse};
use mongodb::Database;

use crate::models::{booking_model::{Booking, BookingRequest}};

#[post("/booking")]
pub async fn create_booking(db: Data<Database>, request: Json<BookingRequest>) -> HttpResponse {
   match db
       .create_booking(
          Booking::try_from(BookingRequest {
              owner: request.owner,
              start_time: request.start_time.clone(),
              duration_in_munites: request.duration_in_munites,
              
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

#[get("/booking")]
  pub async fn get_bookings(db: Data<Database>) -> HttpResponse {
     match db.get_bookings().await {
        Ok(bookings) => {
            HttpResponse::Ok().json(bookings)
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
     }
  }

  #[put("/booking/{id}/cancel")]
  pub async fn cancel_bookings(db: Data<Database>,path: Path<(String)) -> HttpResponse {
     match db.cancel_bookings(id.as_str).await {
        Ok(bookings) => {
            HttpResponse::Ok().json(bookings)
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
     }
  }
