use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::staff::{Staff, StaffInput};
use crate::utils::db::establish_connection;
use crate::models::schema::staff::dsl::{staff, id as staff_id};

pub async fn create_staff(new_staff: web::Json<StaffInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let new_staff = Staff {
        id: Uuid::new_v4(),
        user_id: new_staff.user_id,
        repair_shop_id: new_staff.repair_shop_id,
        role: new_staff.role.clone(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(staff)
        .values(&new_staff)
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Created().json(new_staff),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_staff(path_staff_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = staff.find(*path_staff_id).first::<Staff>(&mut conn);

    match result {
        Ok(other_staff) => HttpResponse::Ok().json(other_staff),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn get_staffs() -> HttpResponse {
    let mut conn = establish_connection();
    let result = staff.load::<Staff>(&mut conn);

    match result {
        Ok(staffs) => HttpResponse::Ok().json(staffs),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_staff(path_staff_id: web::Path<Uuid>, staff_input: web::Json<StaffInput>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = staff.filter(staff_id.eq(*path_staff_id));

    let result = diesel::update(target)
        .set((
            crate::models::schema::staff::user_id.eq(&staff_input.user_id),
            crate::models::schema::staff::repair_shop_id.eq(&staff_input.repair_shop_id),
            crate::models::schema::staff::role.eq(&staff_input.role),
        ))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_staff(path_staff_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = staff.filter(staff_id.eq(*path_staff_id));

    let result = diesel::delete(target).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
