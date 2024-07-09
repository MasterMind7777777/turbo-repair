use actix_web::{web, HttpRequest, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::user_request::{UserRequest, UserRequestInput, UserRequestUpdate};
use crate::utils::db::establish_connection;
use crate::utils::auth::get_user_id_from_token;
use crate::models::schema::user_requests::dsl::{user_requests, id as user_request_id };
use log::{error, info};

pub async fn create_user_request(request: web::Json<UserRequestInput>, req: HttpRequest) -> HttpResponse {
    let user_id = match get_user_id_from_token(&req) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let mut conn = establish_connection();
    let new_request = UserRequest {
        id: Uuid::new_v4(),
        user_id,
        repair_shop_id: request.repair_shop_id,
        content: request.content.clone(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    info!("Creating new user request: {:?}", new_request);

    let result = diesel::insert_into(user_requests)
        .values(&new_request)
        .execute(&mut conn);

    match result {
        Ok(_) => {
            info!("User request created successfully");
            HttpResponse::Created().json(new_request)
        },
        Err(err) => {
            error!("Failed to create user request: {:?}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn get_user_requests() -> HttpResponse {
    let mut conn = establish_connection();
    let result = user_requests.load::<UserRequest>(&mut conn);

    match result {
        Ok(request_list) => HttpResponse::Ok().json(request_list),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_user_request(request_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = user_requests.filter(user_request_id.eq(*request_id))
        .first::<UserRequest>(&mut conn);

    match result {
        Ok(request) => HttpResponse::Ok().json(request),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_user_request(request_id: web::Path<Uuid>, request_input: web::Json<UserRequestUpdate>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = user_requests.filter(user_request_id.eq(*request_id));

    let result = diesel::update(target)
        .set(request_input.into_inner())
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn patch_user_request(request_id: web::Path<Uuid>, request_input: web::Json<UserRequestUpdate>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = user_requests.filter(user_request_id.eq(*request_id));

    let result = diesel::update(target)
        .set(request_input.into_inner())
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_user_request(request_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = user_requests.filter(user_request_id.eq(*request_id));

    let result = diesel::delete(target).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

