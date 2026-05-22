use crate::domain::models::WeightRecord;
use crate::error::AppResult;
use async_trait::async_trait;
use chrono::NaiveDate;

#[async_trait]
pub trait WeightRepository {
    async fn upsert_weight(
        &self,
        record_date: NaiveDate,
        weight_kg: f64,
    ) -> AppResult<WeightRecord>;

    async fn list_weights(&self, limit: u32) -> AppResult<Vec<WeightRecord>>;

    async fn list_weights_between(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> AppResult<Vec<WeightRecord>>;

    async fn update_weight(
        &self,
        record_date: NaiveDate,
        weight_kg: f64,
    ) -> AppResult<Vec<WeightRecord>>;

    async fn delete_weight(&self, record_date: NaiveDate) -> AppResult<Vec<WeightRecord>>;
}
