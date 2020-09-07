use lazy_static::lazy_static;
use mongodb::{Client};

lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

fn create_mongo_client() -> Client {


   let mut client =  Client::with_uri_str("localhost:27017");
   match client{
       Client=>Option::<Client>(client),
   }
}