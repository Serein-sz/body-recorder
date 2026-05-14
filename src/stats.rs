use crate::models::WeightRecord;
use chrono::{Duration, NaiveDate};

const POINT_WINDOW_DAYS: i64 = 7;
const RECENT_AVERAGE_DAYS: i64 = 28;

#[derive(Debug)]
pub struct WeightComparison {
    pub recent_average: PeriodAverage,
    pub points: Vec<ComparisonPoint>,
}

#[derive(Debug)]
pub struct PeriodAverage {
    pub label: &'static str,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub average_kg: Option<f64>,
    pub sample_count: usize,
}

#[derive(Debug)]
pub struct ComparisonPoint {
    pub label: &'static str,
    pub target_date: NaiveDate,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub average_kg: Option<f64>,
    pub sample_count: usize,
    pub delta_from_recent_kg: Option<f64>,
}

pub fn comparison_range(reference_date: NaiveDate) -> (NaiveDate, NaiveDate) {
    (reference_date - Duration::days(372), reference_date)
}

pub fn compare_weights(records: &[WeightRecord], reference_date: NaiveDate) -> WeightComparison {
    let recent_end = reference_date;
    let recent_start = reference_date - Duration::days(RECENT_AVERAGE_DAYS - 1);
    let recent_average = period_average("recent 4 weeks", recent_start, recent_end, records);

    let recent_value = recent_average.average_kg;
    let points = [
        ("1 month ago", reference_date - Duration::days(30)),
        ("6 months ago", reference_date - Duration::days(183)),
        ("1 year ago", reference_date - Duration::days(365)),
    ]
    .into_iter()
    .map(|(label, target_date)| {
        let start = target_date - Duration::days(POINT_WINDOW_DAYS);
        let end = target_date + Duration::days(POINT_WINDOW_DAYS);
        let average = average_between(records, start, end);
        let sample_count = count_between(records, start, end);
        let delta_from_recent_kg = recent_value
            .zip(average)
            .map(|(recent, point)| recent - point);

        ComparisonPoint {
            label,
            target_date,
            start,
            end,
            average_kg: average,
            sample_count,
            delta_from_recent_kg,
        }
    })
    .collect();

    WeightComparison {
        recent_average,
        points,
    }
}

fn period_average(
    label: &'static str,
    start: NaiveDate,
    end: NaiveDate,
    records: &[WeightRecord],
) -> PeriodAverage {
    PeriodAverage {
        label,
        start,
        end,
        average_kg: average_between(records, start, end),
        sample_count: count_between(records, start, end),
    }
}

fn average_between(records: &[WeightRecord], start: NaiveDate, end: NaiveDate) -> Option<f64> {
    let values: Vec<f64> = records
        .iter()
        .filter(|record| record.record_date >= start && record.record_date <= end)
        .map(|record| record.weight_kg)
        .collect();

    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

fn count_between(records: &[WeightRecord], start: NaiveDate, end: NaiveDate) -> usize {
    records
        .iter()
        .filter(|record| record.record_date >= start && record.record_date <= end)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(date: &str, weight_kg: f64) -> WeightRecord {
        WeightRecord {
            record_date: NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
            weight_kg,
            created_at: None,
            updated_at: None,
        }
    }

    #[test]
    fn compares_recent_average_to_prior_windows() {
        let reference_date = NaiveDate::from_ymd_opt(2026, 5, 14).unwrap();
        let records = vec![
            record("2026-05-01", 80.0),
            record("2026-05-14", 78.0),
            record("2026-04-14", 82.0),
            record("2025-11-12", 84.0),
            record("2025-05-14", 86.0),
        ];

        let comparison = compare_weights(&records, reference_date);

        assert_eq!(comparison.recent_average.sample_count, 2);
        assert_eq!(comparison.recent_average.average_kg, Some(79.0));
        assert_eq!(comparison.points[0].average_kg, Some(82.0));
        assert_eq!(comparison.points[0].delta_from_recent_kg, Some(-3.0));
    }

    #[test]
    fn uses_one_year_fetch_range() {
        let reference_date = NaiveDate::from_ymd_opt(2026, 5, 14).unwrap();

        assert_eq!(
            comparison_range(reference_date),
            (NaiveDate::from_ymd_opt(2025, 5, 7).unwrap(), reference_date)
        );
    }
}
