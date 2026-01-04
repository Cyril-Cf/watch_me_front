use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
    Undefined,
}

#[derive(Serialize, Deserialize)]
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

impl User {
    pub fn new(
        id: Uuid,
        email: String,
        password_hash: String,
        name: Option<String>,
        username: Option<String>,
        role: Option<String>,
        first_name: Option<String>,
        gender: Gender,
        birth_ts: Option<i64>,
    ) -> Self {
        Self {
            id,
            email,
            password_hash,
            name,
            username,
            role,
            first_name,
            gender,
            birth_ts,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn username(&self) -> Option<&String> {
        self.username.as_ref()
    }

    pub fn role(&self) -> Option<&String> {
        self.role.as_ref()
    }

    pub fn first_name(&self) -> Option<&String> {
        self.first_name.as_ref()
    }

    pub fn gender(&self) -> &Gender {
        &self.gender
    }

    pub fn birth_ts(&self) -> Option<i64> {
        self.birth_ts
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    email: Option<String>,
    password_hash: Option<String>,
    name: Option<String>,
    username: Option<String>,
    role: Option<String>,
    first_name: Option<String>,
    gender: Gender,
    birth_ts: Option<i64>,
}

impl UpdateUser {
    pub fn new(
        email: Option<String>,
        password_hash: Option<String>,
        name: Option<String>,
        username: Option<String>,
        role: Option<String>,
        first_name: Option<String>,
        gender: Gender,
        birth_ts: Option<i64>,
    ) -> Self {
        Self {
            email,
            password_hash,
            name,
            username,
            role,
            first_name,
            gender,
            birth_ts,
        }
    }
}
