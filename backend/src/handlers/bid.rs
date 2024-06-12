use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::bid::{Bid, BidInput};
use crate::utils::db::establish_connection;
use crate::models::schema::bids::dsl::bids;

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

    let result = diesel::insert_into(bids)
        .values(&new_bid)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().json(new_bid),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_bid(bid_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = bids.filter(crate::models::schema::bids::id.eq(*bid_id))
        .first::<Bid>(&mut conn);

    match result {
        Ok(bid) => HttpResponse::Ok().json(bid),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_bid(bid_id: web::Path<Uuid>, bid_input: web::Json<BidInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = bids.filter(crate::models::schema::bids::id.eq(*bid_id));

    let result = diesel::update(target)
        .set((
            crate::models::schema::bids::bid_amount.eq(&bid_input.bid_amount),
            crate::models::schema::bids::status.eq(&bid_input.status),
        ))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_bid(bid_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = bids.filter(crate::models::schema::bids::id.eq(*bid_id));

    let result = diesel::delete(target).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

