use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::status_pipeline::{StatusPipeline, StatusPipelineInput};
use crate::utils::db::establish_connection;
use crate::models::schema::status_pipeline::dsl::{status_pipeline, id as status_id};

pub async fn get_statuses() -> HttpResponse {
    let mut conn = establish_connection();
    let result = status_pipeline.load::<StatusPipeline>(&mut conn);

    match result {
        Ok(statuses) => HttpResponse::Ok().json(statuses),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_status(new_status: web::Json<StatusPipelineInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let new_status = StatusPipeline {
        id: Uuid::new_v4(),
        order_id: new_status.order_id,
        status: new_status.status.clone(),
        timestamp: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(status_pipeline)
        .values(&new_status)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().json(new_status),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_status(path_status_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = status_pipeline.find(*path_status_id).first::<StatusPipeline>(&mut conn);

    match result {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_status(path_status_id: web::Path<Uuid>, status_input: web::Json<StatusPipelineInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = status_pipeline.filter(status_id.eq(*path_status_id));

    let result = diesel::update(target)
        .set((
            crate::models::schema::status_pipeline::order_id.eq(&status_input.order_id),
            crate::models::schema::status_pipeline::status.eq(&status_input.status),
        ))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_status(path_status_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = status_pipeline.filter(status_id.eq(*path_status_id));

    let result = diesel::delete(target).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

