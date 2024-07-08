use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::order::{Order, OrderInput, OrderStatusUpdate};
use crate::models::status_pipeline::StatusPipeline;
use crate::utils::db::establish_connection;
use crate::models::schema::orders::dsl::orders;
use crate::models::schema::status_pipeline::dsl::status_pipeline;
use crate::models::schema::orders::id;
use crate::models::schema::orders::status;

pub async fn create_order(order: web::Json<OrderInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let new_order = Order {
        id: Uuid::new_v4(),
        repair_request_id: order.repair_request_id,
        repair_shop_id: order.repair_shop_id,
        status: "ongoing".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(orders)
        .values(&new_order)
        .execute(&mut conn);

    match result {
        Ok(_) => {
            let new_status = StatusPipeline {
                id: Uuid::new_v4(),
                order_id: new_order.id,
                status: "ongoing".to_string(),
                timestamp: chrono::Utc::now().naive_utc(),
            };

            diesel::insert_into(status_pipeline)
                .values(&new_status)
                .execute(&mut conn)
                .expect("Error inserting status");

            HttpResponse::Created().json(new_order)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_orders() -> HttpResponse {
    let mut conn = establish_connection();
    let result = orders.load::<Order>(&mut conn);

    match result {
        Ok(order_list) => HttpResponse::Ok().json(order_list),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_order(order_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = orders.filter(crate::models::schema::orders::id.eq(*order_id))
        .first::<Order>(&mut conn);

    match result {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn patch_order_status(order_id: web::Path<Uuid>, order_input: web::Json<OrderStatusUpdate>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = orders.filter(id.eq(*order_id));

    let result = diesel::update(target)
        .set(status.eq(&order_input.status))
        .execute(&mut conn);

    match result {
        Ok(_) => {
            let new_status = StatusPipeline {
                id: Uuid::new_v4(),
                order_id: *order_id,
                status: order_input.status.clone(),
                timestamp: chrono::Utc::now().naive_utc(),
            };

            diesel::insert_into(status_pipeline)
                .values(&new_status)
                .execute(&mut conn)
                .expect("Error inserting status");

            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_order(order_id: web::Path<Uuid>, order_input: web::Json<OrderInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = orders.filter(crate::models::schema::orders::id.eq(*order_id));

    let result = diesel::update(target)
        .set(crate::models::schema::orders::status.eq(&order_input.status))
        .execute(&mut conn);

    match result {
        Ok(_) => {
            let new_status = StatusPipeline {
                id: Uuid::new_v4(),
                order_id: *order_id,
                status: order_input.status.clone(),
                timestamp: chrono::Utc::now().naive_utc(),
            };

            diesel::insert_into(status_pipeline)
                .values(&new_status)
                .execute(&mut conn)
                .expect("Error inserting status");

            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_order(order_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = orders.filter(crate::models::schema::orders::id.eq(*order_id));

    let result = diesel::delete(target).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

