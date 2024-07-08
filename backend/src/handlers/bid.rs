use actix_web::{web, HttpRequest, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;
use crate::models::bid::{Bid, BidInput};
use crate::models::order::Order;
use crate::models::repair_request::RepairRequest;
use crate::models::status_pipeline::StatusPipeline;
use crate::utils::db::establish_connection;
use crate::utils::auth::get_user_id_from_token;
use crate::models::schema::{bids, orders, repair_requests, status_pipeline};

pub async fn create_bid(bid: web::Json<BidInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let new_bid = Bid {
        id: Uuid::new_v4(),
        repair_request_id: bid.repair_request_id,
        repair_shop_id: bid.repair_shop_id,
        bid_amount: bid.bid_amount,
        status: bid.status.clone(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(bids::table)
        .values(&new_bid)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().json(new_bid),
        Err(e) => {
            eprintln!("Error creating bid: {:?}", e);
            HttpResponse::BadRequest().body(format!("Error creating bid: {:?}", e))
        },
    }
}

pub async fn get_bid(bid_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = bids::table.filter(bids::id.eq(*bid_id))
        .first::<Bid>(&mut conn);

    match result {
        Ok(bid) => HttpResponse::Ok().json(bid),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_bid(bid_id: web::Path<Uuid>, bid_input: web::Json<BidInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = bids::table.filter(bids::id.eq(*bid_id));

    let result = diesel::update(target)
        .set((
            bids::bid_amount.eq(&bid_input.bid_amount),
            bids::status.eq(&bid_input.status),
        ))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_bid(bid_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = bids::table.filter(bids::id.eq(*bid_id));

    let result = diesel::delete(target).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Deserialize)]
pub struct BidsQuery {
    pub repair_request_id: Uuid,
}

pub async fn get_bids_for_request(query: web::Query<BidsQuery>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = bids::table
        .filter(bids::repair_request_id.eq(query.repair_request_id))
        .load::<Bid>(&mut conn);

    match result {
        Ok(bids_list) => HttpResponse::Ok().json(bids_list),
        Err(e) => {
            eprintln!("Error fetching bids for request: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Error fetching bids for request: {:?}", e))
        },
    }
}

pub async fn accept_bid(req: HttpRequest, bid_id: web::Path<Uuid>) -> HttpResponse {
    let user_id = match get_user_id_from_token(&req) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let mut conn = establish_connection();

    let bid = match bids::table.filter(bids::id.eq(*bid_id))
        .first::<Bid>(&mut conn) {
        Ok(bid) => bid,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let repair_request = match repair_requests::table
        .filter(repair_requests::id.eq(bid.repair_request_id))
        .first::<RepairRequest>(&mut conn) {
        Ok(request) => request,
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    if repair_request.customer_id != user_id {
        return HttpResponse::Unauthorized().finish();
    }

    let new_order = Order {
        id: Uuid::new_v4(),
        repair_request_id: bid.repair_request_id,
        repair_shop_id: bid.repair_shop_id,
        status: "ongoing".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(orders::table)
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

            diesel::insert_into(status_pipeline::table)
                .values(&new_status)
                .execute(&mut conn)
                .expect("Error inserting status");

            HttpResponse::Created().json(new_order)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

