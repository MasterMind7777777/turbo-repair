use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use super::schema::orders;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: Uuid,
    pub repair_request_id: Uuid,
    pub repair_shop_id: Uuid,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct OrderInput {
    pub repair_request_id: Uuid,
    pub repair_shop_id: Uuid,
    pub status: String,
}

