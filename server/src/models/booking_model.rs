use mongodb::bson::{oid::ObjectId,DateTime};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use chrono::Utc;
use crate::models::{dog_model::Dog, owner_model::Owner};

#[derive(Debug, Deserialize, Serialize)]

pub struct Booking {
     pub _id: ObjectId,
     pub owner: ObjectId,
     pub start_time: DateTime, // Use a proper date/time type in production
     pub duration_in_munites: u8,   // Use a proper date/time type in production
     pub cancelled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookingRequest {
     pub owner: ObjectId,
     pub start_time: DateTime, // Use a proper date/time type in production
     pub duration_in_munites: u8,
}

pub struct FullBooking {
     pub _id: ObjectId,
     pub owner: Owner,
     pub dog: Vec<Dog>, // Assuming a booking can involve multiple dogs
     pub start_time: DateTime, // Use a proper date/time type in production
     pub duration_in_munites: u8,   // Use a proper date/time type in production
     pub cancelled: bool,
}

impl TryFrom<BookingRequest> for Booking {
    type Error = Box<dyn std::error::Error>;
     
     fn try_from(item: BookingRequest) -> Result<Self, Self::Error> {
         let chrono_datetime: SystemTime = chrono::DateTime::parse_from_rfc3339(&item.start_time.to_string())
             .map_err(|err| format!("Failed to parse date: {}", err))?
             .with_timezone(&Utc) 
             .into();

            Ok(Self {
                _id: ObjectId::new(),
                owner: item.owner,
                start_time: DateTime::from(chrono_datetime),
                duration_in_munites: item.duration_in_munites,
                cancelled: false,
            })
     }
    
}