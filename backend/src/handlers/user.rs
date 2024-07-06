use crate::models::schema::users::id as user_id_column;
use crate::utils::auth::get_user_id_from_token;
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::prelude::*;
use log::{ info, error };
use serde::Serialize;
use crate::models::user::{User, UserInput};
use crate::utils::db::establish_connection;
use crate::models::schema::users::dsl::users;
use uuid::Uuid;

use super::auth::hash_password;

pub async fn get_user_details(user_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = users.filter(user_id_column.eq(*user_id))
        .first::<User>(&mut conn);

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    #[serde(with = "uuid::serde::compact")]
    pub id: Uuid,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
        }
    }
}


pub async fn get_current_user(req: HttpRequest) -> HttpResponse {
    // Logging the authorization header
    if let Some(auth_header) = req.headers().get("Authorization") {
        match auth_header.to_str() {
            Ok(header_value) => info!("Authorization header: {}", header_value),
            Err(_) => error!("Failed to parse Authorization header"),
        }
    } else {
        info!("No Authorization header found");
    }

    let user_uuid = match get_user_id_from_token(&req) {
        Ok(id) => {
            info!("Extracted UUID from token: {}", id);
            id
        }
        Err(err) => {
            error!("Failed to extract UUID from token: {:?}", err);
            return HttpResponse::Unauthorized().finish();
        }
    };

    info!("User UUID to search for: {}", user_uuid);

    let mut conn = establish_connection();
    info!("Searching for user with UUID: {}", user_uuid);
    let result = users.find(user_uuid).first::<User>(&mut conn);

    match result {
        Ok(user) => {
            info!("User found: {:?}", user.email);
            HttpResponse::Ok().json(UserResponse::from(user))
        }
        Err(err) => {
            error!("User not found: {:?}", err);
            HttpResponse::NotFound().finish()
        }
    }
}

pub async fn update_user_profile(user_id: web::Path<Uuid>, user_input: web::Json<UserInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = users.filter(user_id_column.eq(*user_id));

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

