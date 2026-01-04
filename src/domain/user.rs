use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
    Undefined,
}

#[derive(Serialize)]
pub struct User {
    id: Uuid,
    email: String,
    password_hash: String,
    name: Option<String>,
    username: Option<String>,
    role: Option<String>,
    first_name: Option<String>,
    gender: Gender,
    birth_ts: Option<i64>,
}
