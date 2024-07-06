use crate::models::schema::{repair_requests as schema_repair_requests, staff, orders as schema_orders};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::http::Error;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use crate::models::repair_request::{RepairRequest, RepairRequestInput};
use crate::utils::db::establish_connection;
use crate::utils::auth::get_user_id_from_token;

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

    let result = diesel::insert_into(schema_repair_requests::table)
        .values(&new_request)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().json(RepairRequestResponse { id: new_request.id.to_string() }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_repair_request(request_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = schema_repair_requests::table
        .filter(schema_repair_requests::id.eq(*request_id))
        .first::<RepairRequest>(&mut conn);

    match result {
        Ok(request) => HttpResponse::Ok().json(request),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[derive(Serialize)]
struct RepairRequestListResponse {
    id: String,
    customer_id: String,
    description: String,
    created_at: chrono::NaiveDateTime,
}

pub async fn list_user_repair_requests(req: HttpRequest) -> Result<HttpResponse, Error> {
    let user_id = match get_user_id_from_token(&req) {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let mut conn = establish_connection();
    let results = schema_repair_requests::table
        .filter(schema_repair_requests::customer_id.eq(user_id))
        .load::<RepairRequest>(&mut conn);

    match results {
        Ok(requests) => {
            let response: Vec<RepairRequestListResponse> = requests.into_iter().map(|req| RepairRequestListResponse {
                id: req.id.to_string(),
                customer_id: req.customer_id.to_string(),
                description: req.description,
                created_at: req.created_at,
            }).collect();
            Ok(HttpResponse::Ok().json(response))
        },
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn list_available_requests(req: HttpRequest) -> Result<HttpResponse, Error> {
    let user_id = match get_user_id_from_token(&req) {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let mut conn = establish_connection();

    // Check if the user is a staff member
    let staff_count = staff::dsl::staff
        .filter(staff::dsl::user_id.eq(user_id))
        .count()
        .get_result::<i64>(&mut conn)
        .expect("Error loading staff count");

    if staff_count == 0 {
        return Ok(HttpResponse::Forbidden().finish());
    }

    // Get available repair requests (i.e., those without orders)
    let results = schema_repair_requests::table
        .left_join(schema_orders::table.on(schema_repair_requests::id.eq(schema_orders::repair_request_id)))
        .filter(schema_orders::id.is_null())
        .select((
            schema_repair_requests::id,
            schema_repair_requests::customer_id,
            schema_repair_requests::description,
            schema_repair_requests::created_at,
        ))
        .load::<(Uuid, Uuid, String, chrono::NaiveDateTime)>(&mut conn);

    match results {
        Ok(requests) => {
            let response: Vec<RepairRequestListResponse> = requests.into_iter().map(|(id, customer_id, description, created_at)| {
                RepairRequestListResponse {
                    id: id.to_string(),
                    customer_id: customer_id.to_string(),
                    description,
                    created_at,
                }
            }).collect();
            Ok(HttpResponse::Ok().json(response))
        },
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn update_repair_request(request_id: web::Path<Uuid>, request_input: web::Json<RepairRequestInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = schema_repair_requests::table.filter(schema_repair_requests::id.eq(*request_id));

    let result = diesel::update(target)
        .set(schema_repair_requests::description.eq(&request_input.description))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_repair_request(request_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = schema_repair_requests::table.filter(schema_repair_requests::id.eq(*request_id));

    let result = diesel::delete(target).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

