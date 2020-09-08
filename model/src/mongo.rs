// use lazy_static::lazy_static;
// use mongodb::{Client};
// use mongodb::coll::Collection;

// lazy_static! {
//     pub static ref MONGO: Client = create_mongo_client();
// }

// fn create_mongo_client() -> Client {
//     Client::connect("localhost", 27017)
//         .expect("Failed to initialize standalone client.")
// }

// fn collection(coll_name: &str) -> Collection {
//     MONGO.db("myblog").collection(coll_name)
// }