use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub supabase_url: String,
    pub service_role_key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WeightRecord {
    pub record_date: NaiveDate,
    pub weight_kg: f64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
