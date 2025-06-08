use mongo_db::bson::{oid::ObjectId,DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]

pub struct Owner {
     pub _id: ObjectId,
     pub name: String,
     pub email: String,
     pub address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Owner_request {
     pub _id: ObjectId,
     pub name: String,
     pub email: String,
     pub address: String,
}

impl TryFrom<Owner_request> for Booking {
    type Error = Box<dyn std::error::Error>;
     
     fn try_from(item: Owner_request) -> Result<Self, Self::Error> {

            Ok(Self {
                _id: ObjectId::new(),
                name: item.name,
                email: item.email,
                address: item.address,
                
            })
     }
    
}