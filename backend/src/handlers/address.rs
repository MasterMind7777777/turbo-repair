use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::address::{Address, AddressInput, PartialAddressInput};
use crate::utils::db::establish_connection;
use crate::models::schema::addresses::dsl::{addresses, id as address_id, repair_shop_id};

pub async fn create_address(address: web::Json<AddressInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let new_address = Address {
        id: Uuid::new_v4(),
        repair_shop_id: address.repair_shop_id,
        street: address.street.clone(),
        city: address.city.clone(),
        state: address.state.clone(),
        zip: address.zip.clone(),
        country: address.country.clone(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(addresses)
        .values(&new_address)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().json(new_address),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_address(path_address_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = addresses.find(*path_address_id).first::<Address>(&mut conn);

    match result {
        Ok(address) => HttpResponse::Ok().json(address),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_address(path_address_id: web::Path<Uuid>, address_input: web::Json<AddressInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = addresses.filter(address_id.eq(*path_address_id));

    let result = diesel::update(target)
        .set((
            crate::models::schema::addresses::street.eq(&address_input.street),
            crate::models::schema::addresses::city.eq(&address_input.city),
            crate::models::schema::addresses::state.eq(&address_input.state),
            crate::models::schema::addresses::zip.eq(&address_input.zip),
            crate::models::schema::addresses::country.eq(&address_input.country),
        ))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn partially_update_address(path_address_id: web::Path<Uuid>, address_input: web::Json<PartialAddressInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = addresses.filter(address_id.eq(*path_address_id));

    let result = diesel::update(target)
        .set(&address_input.into_inner())
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_address(path_address_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = addresses.filter(address_id.eq(*path_address_id));

    let result = diesel::delete(target).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_address_by_shop_id(shop_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = addresses.filter(repair_shop_id.eq(*shop_id)).first::<Address>(&mut conn);

    match result {
        Ok(address) => HttpResponse::Ok().json(address),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
