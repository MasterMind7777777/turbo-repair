use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use uuid::Uuid;
use password_hash::{PasswordHash, PasswordVerifier};
use super::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    #[serde(with = "uuid::serde::compact")]
    pub id: Uuid,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        let parsed_hash = PasswordHash::new(&self.password).unwrap();
        argon2::Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
    }
}

