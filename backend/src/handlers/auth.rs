use actix_web::{web, HttpResponse};
use serde::Deserialize;
use diesel::prelude::*;
use crate::models::user::User;
use crate::utils::db::establish_connection;
use crate::utils::jwt::generate_jwt;
use crate::models::schema::users::dsl::{users, email};

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

pub async fn login(login_input: web::Json<LoginInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = users.filter(email.eq(&login_input.email))
        .first::<User>(&mut conn);

    match result {
        Ok(user) => {
            if user.verify_password(&login_input.password) {
                let token = generate_jwt(&user.id.to_string());
                HttpResponse::Ok().json(token)
            } else {
                HttpResponse::Unauthorized().finish()
            }
        },
        Err(_) => HttpResponse::Unauthorized().finish(),
    }
}

