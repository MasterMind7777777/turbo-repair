use actix_web::{web, HttpRequest, HttpResponse};
use diesel::prelude::*;
use log::{error, info};
use uuid::Uuid;
use crate::models::staff_task::{StaffTask, StaffTaskInput, StaffTaskUpdate, TaskStaffLink, NewStaffTask, LinkStaffInput};
use crate::utils::db::establish_connection;
use crate::utils::auth::{ get_user_id_from_token, get_staff_id_for_user_in_shop };
use crate::models::schema::staff_tasks::dsl::{staff_tasks as staff_tasks_table, id as staff_task_id, content as staff_task_content};
use crate::models::schema::task_staff_links::dsl::task_staff_links as task_staff_links_table;


pub async fn create_staff_task(task: web::Json<StaffTaskInput>, req: HttpRequest) -> HttpResponse {
    let user_id = match get_user_id_from_token(&req) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let mut conn = establish_connection();

    let author_id = match get_staff_id_for_user_in_shop(user_id, task.repair_shop_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };

    let new_task = NewStaffTask {
        author_id,
        repair_shop_id: task.repair_shop_id,
        content: task.content.clone(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    info!("Creating new staff task: {:?}", new_task);

    let result = diesel::insert_into(staff_tasks_table)
        .values(&new_task)
        .get_result::<StaffTask>(&mut conn);

    match result {
        Ok(inserted_task) => {
            for staff_id in &task.staff_ids {
                let new_link = TaskStaffLink {
                    id: Uuid::new_v4(),
                    task_id: inserted_task.id,
                    staff_id: *staff_id,
                };
                diesel::insert_into(task_staff_links_table)
                    .values(&new_link)
                    .execute(&mut conn)
                    .expect("Error linking staff to task");
            }
            HttpResponse::Created().json(inserted_task)
        }
        Err(err) => {
            error!("Failed to create staff task: {:?}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn get_staff_tasks() -> HttpResponse {
    let mut conn = establish_connection();
    let result = staff_tasks_table.load::<StaffTask>(&mut conn);

    match result {
        Ok(task_list) => HttpResponse::Ok().json(task_list),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_staff_task(task_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let result = staff_tasks_table.filter(staff_task_id.eq(*task_id))
        .first::<StaffTask>(&mut conn);

    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn update_staff_task(task_id: web::Path<Uuid>, task_input: web::Json<StaffTaskUpdate>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = staff_tasks_table.filter(staff_task_id.eq(*task_id));

    let result = diesel::update(target)
        .set(staff_task_content.eq(task_input.content.as_ref().unwrap()))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn patch_staff_task(task_id: web::Path<Uuid>, task_input: web::Json<StaffTaskUpdate>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = staff_tasks_table.filter(staff_task_id.eq(*task_id));

    let result = diesel::update(target)
        .set(staff_task_content.eq(task_input.content.as_ref().unwrap()))
        .execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_staff_task(task_id: web::Path<Uuid>) -> HttpResponse {
    let mut conn = establish_connection();
    let target = staff_tasks_table.filter(staff_task_id.eq(*task_id));

    let result = diesel::delete(target).execute(&mut conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn link_staff_to_task(link: web::Json<LinkStaffInput>) -> HttpResponse {
    let mut conn = establish_connection();
    for staff_id in &link.staff_ids {
        let new_link = TaskStaffLink {
            id: Uuid::new_v4(),
            task_id: link.task_id,
            staff_id: *staff_id,
        };

        let result = diesel::insert_into(task_staff_links_table)
            .values(&new_link)
            .execute(&mut conn);

        if result.is_err() {
            return HttpResponse::InternalServerError().finish();
        }
    }

    HttpResponse::Created().finish()
}

