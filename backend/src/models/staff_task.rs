use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use uuid::Uuid;
use super::schema::staff_tasks;
use super::schema::task_staff_links;

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = staff_tasks)]
pub struct StaffTask {
    pub id: Uuid,
    pub author_id: Uuid,
    pub repair_shop_id: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct StaffTaskInput {
    pub repair_shop_id: Uuid,
    pub content: String,
    pub staff_ids: Vec<Uuid>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = staff_tasks)]
pub struct NewStaffTask {
    pub author_id: Uuid,
    pub repair_shop_id: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = staff_tasks)]
pub struct StaffTaskUpdate {
    pub content: Option<String>,
}

#[derive(Queryable, Insertable, Deserialize)]
#[diesel(table_name = task_staff_links)]
pub struct TaskStaffLink {
    pub id: Uuid,
    pub task_id: Uuid,
    pub staff_id: Uuid,
}

#[derive(Deserialize)]
pub struct LinkStaffInput {
    pub task_id: Uuid,
    pub staff_ids: Vec<Uuid>,
}
