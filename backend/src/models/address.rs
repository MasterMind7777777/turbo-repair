use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use super::schema::addresses;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = addresses)]
pub struct Address {
    pub id: Uuid,
    pub repair_shop_id: Uuid,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
    pub created_at: NaiveDateTime,
}

