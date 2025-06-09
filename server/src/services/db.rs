use mongodb::{bson::{doc, from_document, oid::ObjectId, DateTime}, Collection};
use futures_util::stream::StreamExt;
use crate::models::{booking_model::{Booking, FullBooking}, dog_model::Dog, owner_model::Owner};
use std::env;
use mongodb::results::{ UpdateResult, InsertOneResult};
use mongodb::{error::Error};
use std::str::FromStr;
use chrono::Utc;
use std::time::SystemTime;

pub struct Database {
    pub dog : Collection<Dog>,
    pub owner: Collection<Owner>,
    pub booking: Collection<Booking>
}

impl Database  {
    pub async fn init() -> Self {
            let url = match env::var("mongo_url"){
            Ok(v) => v.to_string(),
            Err(_) => "mongodb://localhost:27017".to_string(),
            };
        

        let client = mongodb::Client::with_uri_str(&url).await.unwrap();
        let db = client.database("vent_booking");

        let dog: Collection<Dog> = db.collection("dog");
        let owner: Collection<Owner> = db.collection("owner");
        let booking: Collection<Booking> = db.collection("booking");

        Database  {
            dog,
            owner,
            booking
        }


    }

    pub async fn create_owner(&self, owner: Owner) -> Result<InsertOneResult, Error> {
    let result = self
            .owner
            .insert_one(owner)
            .await
            .ok()
            .expect("Failed to create owner");

            Ok(result)
    }
    pub async fn create_booking(&self, booking: Booking) -> Result<InsertOneResult, Error> {
    let result = self
            .booking
            .insert_one(booking)
            .await
            .ok()
            .expect("Failed to create owner");

            Ok(result)
    }

  

    pub async fn create_dog(&self, dog: Dog) -> Result<InsertOneResult, Error> {
    let result = self
            .dog
            .insert_one(dog)
            .await
            .ok()
            .expect("Failed to create dog");

            Ok(result)
    }

    pub async fn cancel_booking(&self , booking_id: &str) -> Result<UpdateResult, Error> {
        let result = self
        .booking
            .update_one(doc! {
                "_id": ObjectId::from_str(booking_id).expect("failed to parse booking id")
            }, doc! {
                "$set": doc! {
                    "cancelled": true
                }
            })
            .await
            .ok()
            .expect("Failed to cancel booking");
        
        Ok(result)
        }


        pub async fn get_bookings(&self, owner_id: &str) -> Result<Vec<FullBooking>, Error> {
    let now: SystemTime = Utc::now().into();

    let mut result = self
        .booking
        .aggregate(
            vec![
                doc! {
                    "$match": {
                        "owner": ObjectId::from_str(owner_id)
                            .expect("failed to parse owner id"),
                        "start_time": {
                            "$gte": DateTime::from_system_time(now)
                        }
                    }
                },
                doc! {
                    "$lookup": {
                        "from": "owner",
                        "localField": "owner",
                        "foreignField": "_id",
                        "as": "owner_info"
                    }
                },
                doc! {
                    "$unwind": {
                        "path": "$owner_info"
                    }
                },
                doc! {
                    "$lookup": {
                        "from": "dog",
                        "localField": "owner_info._id",
                        "foreignField": "owner",
                        "as": "dog_info"
                    }
                },
            ]
            
        )
        .await?;
       

    let mut bookings: Vec<FullBooking> = Vec::new();

    while let Some(result) = result.next().await {
        match result {
            Ok(doc) => {
                let booking = from_document(doc)
                    .expect("Failed to parse booking document");
                bookings.push(booking);
            }
            Err(err) => {
                return Err(err.into()); // Handle the error gracefully
            }
        }
    }

    Ok(bookings) 
}

    }
 



