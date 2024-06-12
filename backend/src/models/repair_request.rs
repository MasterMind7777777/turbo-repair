use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use super::schema::repair_requests;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = repair_requests)]
pub struct RepairRequest {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub description: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct RepairRequestInput {
    pub customer_id: Uuid,
    pub description: String,
}
