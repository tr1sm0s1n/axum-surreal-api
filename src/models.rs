use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub email: String,
    pub password: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub genre: String,
    description: String,
    avg_rating: u8,
    pub reviews: Vec<Review>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Review {
    pub user: String,
    title: String,
    content: String,
    rating: u8
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}
