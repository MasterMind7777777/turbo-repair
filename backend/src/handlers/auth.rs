use actix_web::{web, HttpResponse};
use log::info;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error as DieselError;
use serde_json::json;
use uuid::Uuid;
use crate::models::user::{User, UserInput};
use crate::utils::db::establish_connection;
use crate::utils::jwt::generate_jwt;
use crate::models::schema::users::dsl::{users, email};
use argon2::{Argon2, PasswordHasher};
use password_hash::SaltString;
use rand_core::OsRng;

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
struct RegisterResponse {
    user_id: String,
    token: String,
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub async fn register_user(user: web::Json<UserInput>) -> HttpResponse {
    info!("Received request to register user: {:?}", user);
    let mut conn = establish_connection();
    let new_user = User {
        id: Uuid::new_v4(),
        email: user.email.clone(),
        password: hash_password(&user.password),
    };

    match diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut conn) {
        Ok(_) => {
            let token = generate_jwt(&new_user.id.to_string());
            info!("User registered successfully");
            HttpResponse::Ok().json(json!({
                "user_id": new_user.id.to_string(),
                "token": token
            }))
        },
        Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info)) => {
            let constraint = info.constraint_name().unwrap_or("unknown");
            info!("Unique constraint violation on {}", constraint);
            HttpResponse::BadRequest().json(format!("Unique constraint violation on {}", constraint))
        },
        Err(e) => {
            info!("Error saving new user: {:?}", e);
            HttpResponse::InternalServerError().json("Error saving new user")
        }
    }
}

pub async fn login(login_input: web::Json<LoginInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = users.filter(email.eq(&login_input.email))
        .first::<User>(&mut conn);

    match result {
        Ok(user) => {
            if user.verify_password(&login_input.password) {
                let token = generate_jwt(&user.id.to_string());
                HttpResponse::Ok().json(json!({ "token": token, "user_id": user.id }))
            } else {
                HttpResponse::Unauthorized().finish()
            }
        },
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

