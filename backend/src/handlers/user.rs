use crate::models::schema::users::id;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use crate::models::user::{User, UserInput};
use crate::utils::db::establish_connection;
use crate::models::schema::users::dsl::users;
use uuid::Uuid;

use super::auth::hash_password;


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


