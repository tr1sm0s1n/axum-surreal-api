use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub email: String,
    pub password: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}
