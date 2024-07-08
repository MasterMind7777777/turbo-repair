use actix_web::{web, HttpRequest, HttpResponse};
use crate::models::schema::staff::dsl::staff;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::repair_shop::{PartialRepairShopInput, RepairShop, RepairShopInput};
use crate::models::staff::Staff;
use crate::utils::auth::get_user_id_from_token;
use crate::utils::db::establish_connection;
use crate::models::schema::repair_shops::dsl::repair_shops;

pub async fn create_repair_shop(
    req: HttpRequest,
    shop: web::Json<RepairShopInput>,
) -> HttpResponse {
    let user_id = match get_user_id_from_token(&req) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let mut conn = establish_connection();

    let new_shop = RepairShop {
        id: Uuid::new_v4(),
        name: shop.name.clone(),
        created_at: Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(repair_shops)
        .values(&new_shop)
        .execute(&mut conn);

    match result {
        Ok(_) => {
            let new_staff = Staff {
                id: Uuid::new_v4(),
                user_id,
                repair_shop_id: new_shop.id,
                role: "manager".to_string(),
                created_at: Utc::now().naive_utc(),
            };

            let staff_result = diesel::insert_into(staff)
                .values(&new_staff)
                .execute(&mut conn);

            match staff_result {
                Ok(_) => HttpResponse::Created().json(new_shop),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
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
        Ok(count) => {
            if count == 0 {
                HttpResponse::NotFound().finish()
            } else {
                HttpResponse::Ok().finish()
            }
        }
        Err(e) => {
            log::error!("Failed to delete repair shop: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
