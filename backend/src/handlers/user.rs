use crate::models::schema::users::id;
use actix_web::{web, HttpResponse};
use log::info;
use diesel::prelude::*;
use crate::models::user::{User, UserInput};
use crate::utils::db::establish_connection;
use crate::models::schema::users::dsl::users;
use uuid::Uuid;
use argon2::{Argon2, PasswordHasher};
use password_hash::SaltString;
use rand_core::OsRng;

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
            info!("User registered successfully");
            HttpResponse::Ok().json("User registered successfully")
        },
        Err(e) => {
            info!("Error saving new user: {:?}", e);
            HttpResponse::InternalServerError().json("Error saving new user")
        }
    }
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub async fn get_user_details(user_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = users.filter(id.eq(*user_id))
        .first::<User>(&mut conn);

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_user_profile(user_id: web::Path<Uuid>, user_input: web::Json<UserInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = users.filter(id.eq(*user_id));

    let result = diesel::update(target)
        .set((
            crate::models::schema::users::email.eq(&user_input.email),
            crate::models::schema::users::password.eq(hash_password(&user_input.password)),
        ))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


