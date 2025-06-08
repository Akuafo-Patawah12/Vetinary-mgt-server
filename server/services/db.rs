use mongodb::Collection;
use crate::models::{booking_model::Booking, dog_model::Dog, owner_model::Owner};


pub struct database {
    pub dog : Collection<Dog>,
    pub owner: Collection<Owner>,
    pub booking: Collection<Booking>
}

impl database {
    pub async fn init() -> Self {
        let url = match env::var("mongo_url"){
        Ok(v) => v::to_string(),
        Err(_) => "mongodb://localhost:27017".to_string(),
        };
    

    let client = mongodb::Client::with_uri_str(&url).await.unwrap();
    let db = client.database("vent_booking");

    let dog: Collection<Dog> = db.collection("dog");
    let owner: Collection<Owner> = db.collection("owner");
    let booking: Collection<Booking> = db.collection("booking");

    database {
        dog,
        owner,
        booking
    }
 }
}

