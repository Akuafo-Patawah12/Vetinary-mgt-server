use mongo_db::bson::{oid::ObjectId,DateTime}
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]

pub struct Dog {
     pub _id: ObjectId,
     pub owner: ObjectId,
     pub name: Option<String>,
     pub age: u8,
     pub address: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dog_request {
     pub owner: ObjectId,
     pub name: Option<String>,
     pub age: Option<u8>,
     pub breed: Option<String>,
}

impl TryFrom<Dog_request> for Booking {
    type Error = Box<dyn std::error::Error>;
     
     fn try_from(item: Dog_request) -> Result<Self, Self::Error> {

            Ok(Self {
                pub _id: ObjectId::new(),
                pub owner: ObjectId::parser_str(&item.owner).expect("Failed to parse owner"),
                pub name: item.name,
                pub age: item.age,
                pub breed: item.breed,
                
            })
     }
    
}