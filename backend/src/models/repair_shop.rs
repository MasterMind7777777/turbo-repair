use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use super::schema::repair_shops;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = repair_shops)]
pub struct RepairShop {
    pub id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct RepairShopInput {
    pub name: String,
}

