use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use super::schema::status_pipeline;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = status_pipeline)]
pub struct StatusPipeline {
    pub id: Uuid,
    pub order_id: Uuid,
    pub status: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = status_pipeline)]
pub struct StatusPipelineInput {
    pub order_id: Uuid,
    pub status: String,
}
