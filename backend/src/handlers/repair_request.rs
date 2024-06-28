use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use crate::models::repair_request::{RepairRequest, RepairRequestInput};
use crate::utils::db::establish_connection;
use crate::models::schema::repair_requests::dsl::repair_requests;

#[derive(Serialize)]
struct RepairRequestResponse {
    id: String,
}

pub async fn create_repair_request(request: web::Json<RepairRequestInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let new_request = RepairRequest {
        id: Uuid::new_v4(),
        customer_id: request.customer_id,
        description: request.description.clone(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(repair_requests)
        .values(&new_request)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().json(RepairRequestResponse { id: new_request.id.to_string() }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_repair_request(request_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = repair_requests.filter(crate::models::schema::repair_requests::id.eq(*request_id))
        .first::<RepairRequest>(&mut conn);

    match result {
        Ok(request) => HttpResponse::Ok().json(request),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_repair_request(request_id: web::Path<Uuid>, request_input: web::Json<RepairRequestInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = repair_requests.filter(crate::models::schema::repair_requests::id.eq(*request_id));

    let result = diesel::update(target)
        .set(crate::models::schema::repair_requests::description.eq(&request_input.description))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_repair_request(request_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = repair_requests.filter(crate::models::schema::repair_requests::id.eq(*request_id));

    let result = diesel::delete(target).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

