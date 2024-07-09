use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use uuid::Uuid;
use super::schema::user_requests;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_requests)]
pub struct UserRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub repair_shop_id: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = user_requests)]
pub struct UserRequestInput {
    pub repair_shop_id: Uuid,
    pub content: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = user_requests)]
pub struct UserRequestUpdate {
    pub content: Option<String>,
}
