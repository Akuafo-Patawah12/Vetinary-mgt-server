use actix_web::{ get, post, put, web::{Data, Json, Query, Path}, HttpResponse};

use crate::services::db::Database;

use crate::models::{booking_model::{Booking, BookingRequest, BookingQuery}};

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
       Ok(booking) => HttpResponse::Ok().json(booking),
         Err(err) => {
              eprintln!("Error creating dog: {}", err);
              HttpResponse::InternalServerError().body(err.to_string())
         }
        }
}

#[get("/booking")]
  pub async fn get_bookings(db: Data<Database>, query: Query<BookingQuery>) -> HttpResponse {
     match db.get_bookings(&query.owner_id).await {
        Ok(bookings) => {
            HttpResponse::Ok().json(bookings)
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
     }
  }

  #[put("/booking/{id}/cancel")]
  pub async fn cancel_bookings(db: Data<Database>,path: Path<(String,)>) -> HttpResponse {
     let (id,) = path.into_inner(); // destructure the tuple
     match db.cancel_booking(id.as_str()).await {
        Ok(bookings) => {
            HttpResponse::Ok().json(bookings)
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(err.to_string())
        }
     }
  }
