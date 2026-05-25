use crate::domain::goals::AdviceGoal;
use crate::domain::models::WeightRecord;
use crate::domain::stats::{
    DietAdvice, DietGoal, TargetProjection, TdeeEstimate, WeightComparison, advice_range,
    build_diet_advice, build_target_projection, build_tdee_estimate, compare_weights,
    comparison_range, tdee_range,
};
use crate::domain::validation::{parse_date, parse_or_today, validate_weight};
use crate::error::AppResult;
use crate::storage::repository::WeightRepository;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct AddWeightResult {
    pub record: WeightRecord,
}

#[derive(Debug)]
pub struct ListWeightsResult {
    pub records: Vec<WeightRecord>,
}

#[derive(Debug)]
pub struct UpdateWeightResult {
    pub date: NaiveDate,
    pub record: Option<WeightRecord>,
}

#[derive(Debug)]
pub struct DeleteWeightResult {
    pub date: NaiveDate,
    pub deleted: bool,
}

#[derive(Debug)]
pub struct CompareWeightsResult {
    pub reference_date: NaiveDate,
    pub total_records: usize,
    pub comparison: WeightComparison,
}

#[derive(Debug)]
pub struct AdviceResult {
    pub advice: DietAdvice,
}

#[derive(Debug)]
pub struct TargetResult {
    pub projection: TargetProjection,
}

#[derive(Debug)]
pub struct TdeeResult {
    pub estimate: TdeeEstimate,
}

pub async fn add_weight(
    repository: &impl WeightRepository,
    weight_kg: f64,
    date: Option<String>,
) -> AppResult<AddWeightResult> {
    let date = parse_or_today(date)?;
    let record = repository
        .upsert_weight(date, validate_weight(weight_kg)?)
        .await?;

    Ok(AddWeightResult { record })
}

pub async fn list_weights(
    repository: &impl WeightRepository,
    limit: u32,
) -> AppResult<ListWeightsResult> {
    let records = repository.list_weights(limit).await?;

    Ok(ListWeightsResult { records })
}

pub async fn update_weight(
    repository: &impl WeightRepository,
    date: String,
    weight_kg: f64,
) -> AppResult<UpdateWeightResult> {
    let date = parse_date(&date)?;
    let records = repository
        .update_weight(date, validate_weight(weight_kg)?)
        .await?;

    Ok(UpdateWeightResult {
        date,
        record: records.into_iter().next(),
    })
}

pub async fn delete_weight(
    repository: &impl WeightRepository,
    date: String,
) -> AppResult<DeleteWeightResult> {
    let date = parse_date(&date)?;
    let deleted = !repository.delete_weight(date).await?.is_empty();

    Ok(DeleteWeightResult { date, deleted })
}

pub async fn compare(
    repository: &impl WeightRepository,
    date: Option<String>,
) -> AppResult<CompareWeightsResult> {
    let reference_date = parse_or_today(date)?;
    let (start, end) = comparison_range(reference_date);
    let records = repository.list_weights_between(start, end).await?;
    let total_records = records.len();
    let comparison = compare_weights(&records, reference_date);

    Ok(CompareWeightsResult {
        reference_date,
        total_records,
        comparison,
    })
}

pub async fn advice(
    repository: &impl WeightRepository,
    goal: Option<AdviceGoal>,
    date: Option<String>,
) -> AppResult<AdviceResult> {
    let reference_date = parse_or_today(date)?;
    let (start, end) = advice_range(reference_date);
    let records = repository.list_weights_between(start, end).await?;
    let goal = goal.unwrap_or(AdviceGoal::Cut);
    let advice = build_diet_advice(&records, reference_date, goal.into());

    Ok(AdviceResult { advice })
}

pub async fn target(
    repository: &impl WeightRepository,
    target_kg: f64,
    date: Option<String>,
) -> AppResult<TargetResult> {
    let reference_date = parse_or_today(date)?;
    let target_kg = validate_weight(target_kg)?;
    let (start, end) = advice_range(reference_date);
    let records = repository.list_weights_between(start, end).await?;
    let projection = build_target_projection(&records, reference_date, target_kg);

    Ok(TargetResult { projection })
}

pub async fn tdee(
    repository: &impl WeightRepository,
    date: Option<String>,
) -> AppResult<TdeeResult> {
    let reference_date = parse_or_today(date)?;
    let (start, end) = tdee_range(reference_date);
    let records = repository.list_weights_between(start, end).await?;
    let estimate = build_tdee_estimate(&records, reference_date);

    Ok(TdeeResult { estimate })
}

impl From<AdviceGoal> for DietGoal {
    fn from(value: AdviceGoal) -> Self {
        match value {
            AdviceGoal::Cut => Self::Cut,
            AdviceGoal::Maintain => Self::Maintain,
            AdviceGoal::Gain => Self::Gain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct FakeRepository {
        records: Mutex<Vec<WeightRecord>>,
        calls: Mutex<Vec<String>>,
    }

    impl FakeRepository {
        fn new(records: Vec<WeightRecord>) -> Self {
            Self {
                records: Mutex::new(records),
                calls: Mutex::new(Vec::new()),
            }
        }

        fn calls(&self) -> Vec<String> {
            self.calls.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl WeightRepository for FakeRepository {
        async fn upsert_weight(
            &self,
            record_date: NaiveDate,
            weight_kg: f64,
        ) -> AppResult<WeightRecord> {
            self.calls
                .lock()
                .unwrap()
                .push(format!("upsert:{record_date}:{weight_kg:.2}"));
            Ok(record(record_date, weight_kg))
        }

        async fn list_weights(&self, limit: u32) -> AppResult<Vec<WeightRecord>> {
            self.calls.lock().unwrap().push(format!("list:{limit}"));
            Ok(self
                .records
                .lock()
                .unwrap()
                .iter()
                .take(limit as usize)
                .cloned()
                .collect())
        }

        async fn list_weights_between(
            &self,
            start: NaiveDate,
            end: NaiveDate,
        ) -> AppResult<Vec<WeightRecord>> {
            self.calls
                .lock()
                .unwrap()
                .push(format!("between:{start}:{end}"));
            Ok(self
                .records
                .lock()
                .unwrap()
                .iter()
                .filter(|record| record.record_date >= start && record.record_date <= end)
                .cloned()
                .collect())
        }

        async fn update_weight(
            &self,
            record_date: NaiveDate,
            weight_kg: f64,
        ) -> AppResult<Vec<WeightRecord>> {
            self.calls
                .lock()
                .unwrap()
                .push(format!("update:{record_date}:{weight_kg:.2}"));
            Ok(self
                .records
                .lock()
                .unwrap()
                .iter()
                .filter(|record| record.record_date == record_date)
                .map(|_| record(record_date, weight_kg))
                .collect())
        }

        async fn delete_weight(&self, record_date: NaiveDate) -> AppResult<Vec<WeightRecord>> {
            self.calls
                .lock()
                .unwrap()
                .push(format!("delete:{record_date}"));
            Ok(self
                .records
                .lock()
                .unwrap()
                .iter()
                .filter(|record| record.record_date == record_date)
                .cloned()
                .collect())
        }
    }

    fn date(value: &str) -> NaiveDate {
        NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap()
    }

    fn record(record_date: NaiveDate, weight_kg: f64) -> WeightRecord {
        WeightRecord {
            record_date,
            weight_kg,
            created_at: None,
            updated_at: None,
        }
    }

    #[tokio::test]
    async fn add_weight_validates_input_before_storage_call() {
        let repository = FakeRepository::new(Vec::new());
        let error = add_weight(&repository, 0.0, Some("2026-05-14".to_string()))
            .await
            .unwrap_err();

        assert!(matches!(error, AppError::InvalidWeight(0.0)));
        assert!(repository.calls().is_empty());
    }

    #[tokio::test]
    async fn add_weight_uses_repository_for_valid_input() {
        let repository = FakeRepository::new(Vec::new());
        let result = add_weight(&repository, 72.45, Some("2026-05-14".to_string()))
            .await
            .unwrap();

        assert_eq!(result.record, record(date("2026-05-14"), 72.45));
        assert_eq!(repository.calls(), ["upsert:2026-05-14:72.45"]);
    }

    #[tokio::test]
    async fn update_weight_reports_missing_record_without_network_transport() {
        let repository = FakeRepository::new(Vec::new());
        let result = update_weight(&repository, "2026-05-14".to_string(), 72.0)
            .await
            .unwrap();

        assert_eq!(result.date, date("2026-05-14"));
        assert_eq!(result.record, None);
        assert_eq!(repository.calls(), ["update:2026-05-14:72.00"]);
    }

    #[tokio::test]
    async fn compare_fetches_expected_range_from_repository() {
        let reference_date = date("2026-05-14");
        let repository = FakeRepository::new(vec![record(reference_date, 72.0)]);
        let result = compare(&repository, Some("2026-05-14".to_string()))
            .await
            .unwrap();

        assert_eq!(result.reference_date, reference_date);
        assert_eq!(result.total_records, 1);
        assert_eq!(repository.calls(), ["between:2025-05-14:2026-05-14"]);
    }

    #[tokio::test]
    async fn target_fetches_recent_trend_range_from_repository() {
        let reference_date = date("2026-05-28");
        let repository = FakeRepository::new(vec![record(reference_date, 72.0)]);
        let result = target(&repository, 70.0, Some("2026-05-28".to_string()))
            .await
            .unwrap();

        assert_eq!(result.projection.target_kg, 70.0);
        assert_eq!(repository.calls(), ["between:2026-05-01:2026-05-28"]);
    }

    #[tokio::test]
    async fn tdee_fetches_latest_seven_day_range_from_repository() {
        let reference_date = date("2026-05-25");
        let repository = FakeRepository::new(vec![
            record(date("2026-05-19"), 70.0),
            record(date("2026-05-21"), 71.0),
            record(reference_date, 72.0),
        ]);
        let result = tdee(&repository, Some("2026-05-25".to_string()))
            .await
            .unwrap();

        assert_eq!(result.estimate.reference_date, reference_date);
        assert_eq!(result.estimate.sample_count, 3);
        assert_eq!(result.estimate.average_weight_kg, Some(71.0));
        assert_eq!(repository.calls(), ["between:2026-05-19:2026-05-25"]);
    }

    #[tokio::test]
    async fn tdee_returns_low_sample_without_error() {
        let reference_date = date("2026-05-25");
        let repository = FakeRepository::new(vec![record(reference_date, 72.0)]);
        let result = tdee(&repository, Some("2026-05-25".to_string()))
            .await
            .unwrap();

        assert_eq!(result.estimate.sample_count, 1);
        assert!(result.estimate.tdee_kcal.is_some());
        assert_eq!(repository.calls(), ["between:2026-05-19:2026-05-25"]);
    }

    #[tokio::test]
    async fn tdee_returns_no_data_without_error() {
        let repository = FakeRepository::new(Vec::new());
        let result = tdee(&repository, Some("2026-05-25".to_string()))
            .await
            .unwrap();

        assert_eq!(result.estimate.sample_count, 0);
        assert_eq!(result.estimate.tdee_kcal, None);
        assert_eq!(repository.calls(), ["between:2026-05-19:2026-05-25"]);
    }
}
