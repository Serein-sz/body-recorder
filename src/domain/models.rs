use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserProfile {
    pub height_cm: f64,
    pub sex: String,
    pub birth_date: NaiveDate,
    pub activity_factor: f64,
    pub target_weight_kg: f64,
    pub fat_loss_training_band: String,
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            height_cm: 173.0,
            sex: "male".to_string(),
            birth_date: NaiveDate::from_ymd_opt(2001, 3, 6).unwrap(),
            activity_factor: 1.60,
            target_weight_kg: 70.0,
            fat_loss_training_band: "six_to_seven_hours".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub supabase_url: String,
    pub service_role_key: String,
    #[serde(default)]
    pub profile: UserProfile,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WeightRecord {
    pub record_date: NaiveDate,
    pub weight_kg: f64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
