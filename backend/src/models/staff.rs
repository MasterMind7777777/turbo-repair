use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use super::schema::staff;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = staff)]
pub struct Staff {
    pub id: Uuid,
    pub user_id: Uuid,
    pub repair_shop_id: Uuid,
    pub role: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = staff)]
pub struct StaffInput {
    pub user_id: Uuid,
    pub repair_shop_id: Uuid,
    pub role: String,
}
