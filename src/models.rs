use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub genre: String,
    description: String,
    pub avg_rating: Option<u8>,
    pub reviews: Option<Vec<Review>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Review {
    pub user: String,
    pub title: String,
    content: String,
    pub rating: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}
