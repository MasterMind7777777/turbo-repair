use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use super::schema::bids;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = bids)]
pub struct Bid {
    pub id: Uuid,
    pub repair_request_id: Uuid,
    pub repair_shop_id: Uuid,
    pub bid_amount: f64,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct BidInput {
    pub repair_request_id: Uuid,
    pub repair_shop_id: Uuid,
    pub bid_amount: f64,
    pub status: String,
}

