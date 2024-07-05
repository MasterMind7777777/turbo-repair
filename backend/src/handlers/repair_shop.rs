use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::repair_shop::{PartialRepairShopInput, RepairShop, RepairShopInput};
use crate::utils::db::establish_connection;
use crate::models::schema::repair_shops::dsl::repair_shops;

pub async fn create_repair_shop(shop: web::Json<RepairShopInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let new_shop = RepairShop {
        id: Uuid::new_v4(),
        name: shop.name.clone(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(repair_shops)
        .values(&new_shop)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().json(new_shop),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_repair_shop(shop_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = repair_shops.filter(crate::models::schema::repair_shops::id.eq(*shop_id))
        .first::<RepairShop>(&mut conn);

    match result {
        Ok(shop) => HttpResponse::Ok().json(shop),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn get_repair_shops() -> HttpResponse {
    let mut conn = establish_connection();
    let results = repair_shops
        .load::<RepairShop>(&mut conn);

    match results {
        Ok(shops) => HttpResponse::Ok().json(shops),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_repair_shop(shop_id: web::Path<Uuid>, shop_input: web::Json<RepairShopInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = repair_shops.filter(crate::models::schema::repair_shops::id.eq(*shop_id));

    let result = diesel::update(target)
        .set(crate::models::schema::repair_shops::name.eq(&shop_input.name))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn partially_update_repair_shop(shop_id: web::Path<Uuid>, shop_input: web::Json<PartialRepairShopInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = repair_shops.filter(crate::models::schema::repair_shops::id.eq(*shop_id));

    let result = diesel::update(target)
        .set(&shop_input.into_inner())
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_repair_shop(shop_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = repair_shops.filter(crate::models::schema::repair_shops::id.eq(*shop_id));

    let result = diesel::delete(target).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

