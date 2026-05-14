use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub supabase_url: String,
    pub service_role_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WeightRecord {
    pub record_date: NaiveDate,
    pub weight_kg: f64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WeightPayload {
    pub record_date: NaiveDate,
    pub weight_kg: f64,
}

#[derive(Debug, Serialize)]
pub struct WeightUpdatePayload {
    pub weight_kg: f64,
}
