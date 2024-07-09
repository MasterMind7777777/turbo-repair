use actix_web::HttpRequest;
use diesel::prelude::*;
use jsonwebtoken::{decode, Validation, DecodingKey, errors::Error as JwtError};
use uuid::Uuid;
use std::env;
use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::utils::db::establish_connection; // Ensure serde is imported

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Missing or invalid Authorization header")]
    InvalidAuthHeader,
    #[error("JWT decoding error: {0}")]
    JwtDecodeError(#[from] JwtError),
    #[error("UUID parsing error")]
    UuidParseError,
    #[error("Environment variable error")]
    EnvVarError(#[from] std::env::VarError),
}

pub fn get_user_id_from_token(req: &HttpRequest) -> Result<Uuid, AuthError> {
    let headers = req.headers();
    let token = match headers.get("Authorization") {
        Some(header_value) => {
            let auth_header = header_value.to_str().map_err(|_| AuthError::InvalidAuthHeader)?;
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                token
            } else {
                return Err(AuthError::InvalidAuthHeader);
            }
        },
        None => return Err(AuthError::InvalidAuthHeader),
    };

    // Load the JWT_SECRET from the environment
    dotenv::dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET")?;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    ).map_err(AuthError::JwtDecodeError)?;

    let user_id = Uuid::parse_str(&token_data.claims.sub).map_err(|_| AuthError::UuidParseError)?;
    Ok(user_id)
}

pub fn get_staff_id_for_user_in_shop(user_id: Uuid, shop_id: Uuid) -> Result<Uuid, diesel::result::Error> {
    use crate::models::schema::staff::dsl::{staff, user_id as staff_user_id, repair_shop_id as staff_repair_shop_id, id as staff_id};
    let mut conn = establish_connection();

    staff
        .filter(staff_user_id.eq(user_id))
        .filter(staff_repair_shop_id.eq(shop_id))
        .select(staff_id)
        .first::<Uuid>(&mut conn)
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (user id)
    exp: usize, // Expiration time
}

