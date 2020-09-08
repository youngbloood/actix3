use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Article {
    pub _id: Option<ObjectId>,
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Article {
    pub const TABLE_NAME: &'static str = "article";
}
