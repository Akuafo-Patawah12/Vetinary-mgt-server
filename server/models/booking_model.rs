use mongo_db::bson::{oid::ObjectId,DateTime}
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]

pub struct Booking {
     pub _id: ObjectId,
     pub owner: ObjectId,
     pub room_id: i32,
     pub start_time: DateTime, // Use a proper date/time type in production
     pub duration _in_munites: u8,   // Use a proper date/time type in production
     pub cancelled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Booking_request {
     pub owner: ObjectId,
     pub start_time: DateTime, // Use a proper date/time type in production
     pub duration _in_munites: u8,
}

impl TryFrom<Booking_request> for Booking {
    type Error = Box<dyn std::error::Error>;
     
     fn try_from(item: Booking_request) -> Result<Self, Self::Error> {
         let chrono_datetime: SystemTime = chrono::DateTime::parse_from_rfc3339(&item.start_time.to_string())
             .map_err(|err| format!("Failed to parse date: {}", err))?
             .with_timezone(&Utc) 
             .into();

            Ok(Self {
                id: ObjectId::new(),
                owner: ObjectId::parse_str(&item.owner).expect("Failed to parse owner"),
                start_time: DateTime::from(chrono_datetime),
                duration_in_minutes: item.duration_in_minutes,
                cancelled: false,
            })
     }
    
}