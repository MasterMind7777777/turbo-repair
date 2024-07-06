use actix_web::{web, Error, HttpRequest, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::staff::{Staff, StaffInput};
use crate::utils::auth::get_user_id_from_token;
use crate::utils::db::establish_connection;
use crate::models::schema::{staff, staff::dsl::staff as all_staff};

pub async fn create_staff(new_staff: web::Json<StaffInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let new_staff = Staff {
        id: Uuid::new_v4(),
        user_id: new_staff.user_id,
        repair_shop_id: new_staff.repair_shop_id,
        role: new_staff.role.clone(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(all_staff)
        .values(&new_staff)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().json(new_staff),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_staff(path_staff_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = all_staff.find(*path_staff_id).first::<Staff>(&mut conn);

    match result {
        Ok(other_staff) => HttpResponse::Ok().json(other_staff),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[derive(Deserialize)]
pub struct AddStaffInput {
    user_id: Uuid,
    repair_shop_id: Uuid,
    role: String,
}

#[derive(Serialize)]
struct AddStaffResponse {
    id: Uuid,
}

pub async fn add_staff(req: HttpRequest, staff_input: web::Json<AddStaffInput>) -> Result<HttpResponse, Error> {
    let requester_user_id = match get_user_id_from_token(&req) {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };

    let mut conn = establish_connection();

    // Verify that the requester is an existing staff member of the given repair shop
    let staff_count = all_staff
        .filter(staff::user_id.eq(requester_user_id))
        .filter(staff::repair_shop_id.eq(staff_input.repair_shop_id))
        .count()
        .get_result::<i64>(&mut conn)
        .expect("Error loading staff count");

    if staff_count == 0 {
        return Ok(HttpResponse::Forbidden().finish());
    }

    // Create the new staff entry
    let new_staff = Staff {
        id: Uuid::new_v4(),
        user_id: staff_input.user_id,
        repair_shop_id: staff_input.repair_shop_id,
        role: staff_input.role.clone(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(all_staff)
        .values(&new_staff)
        .execute(&mut conn);

    match result {
        Ok(_) => Ok(HttpResponse::Created().json(AddStaffResponse { id: new_staff.id })),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn get_staffs() -> HttpResponse {
    let mut conn = establish_connection();
    let result = all_staff.load::<Staff>(&mut conn);

    match result {
        Ok(staffs) => HttpResponse::Ok().json(staffs),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_staff(path_staff_id: web::Path<Uuid>, staff_input: web::Json<StaffInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = all_staff.filter(staff::id.eq(*path_staff_id));

    let result = diesel::update(target)
        .set((
            staff::user_id.eq(&staff_input.user_id),
            staff::repair_shop_id.eq(&staff_input.repair_shop_id),
            staff::role.eq(&staff_input.role),
        ))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_staff(path_staff_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = all_staff.filter(staff::id.eq(*path_staff_id));

    let result = diesel::delete(target).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

