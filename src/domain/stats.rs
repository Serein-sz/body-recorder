use crate::domain::models::WeightRecord;
use chrono::{Duration, NaiveDate};

const POINT_WINDOW_DAYS: i64 = 7;
const RECENT_AVERAGE_DAYS: i64 = 28;
const ADVICE_WINDOW_DAYS: i64 = 28;
const ADVICE_ENDPOINT_DAYS: i64 = 7;
const MIN_ADVICE_RECORDS: usize = 10;
const MIN_ENDPOINT_RECORDS: usize = 3;
pub const HEIGHT_METERS: f64 = 1.73;
pub const DEFAULT_TARGET_WEIGHT_KG: f64 = 70.0;

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
    pub value_source: ComparisonValueSource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComparisonValueSource {
    Direct,
    Filled,
    Missing,
}

impl ComparisonValueSource {
    pub fn label(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Filled => "filled",
            Self::Missing => "missing",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BmiCategory {
    Underweight,
    Normal,
    Overweight,
    Obesity,
}

impl BmiCategory {
    pub fn label(self) -> &'static str {
        match self {
            Self::Underweight => "underweight",
            Self::Normal => "normal",
            Self::Overweight => "overweight",
            Self::Obesity => "obesity",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DietGoal {
    Cut,
    Maintain,
    Gain,
}

impl DietGoal {
    pub fn label(self) -> &'static str {
        match self {
            Self::Cut => "fat loss",
            Self::Maintain => "maintenance",
            Self::Gain => "weight gain",
        }
    }
}

#[derive(Debug)]
pub struct DietAdvice {
    pub goal: DietGoal,
    pub analysis: TrendAnalysis,
    pub recommendation: Option<AdviceRecommendation>,
    pub interpretation: &'static str,
}

#[derive(Debug)]
pub struct TargetProjection {
    pub target_kg: f64,
    pub analysis: TrendAnalysis,
    pub current_average_kg: Option<f64>,
    pub remaining_kg: Option<f64>,
    pub estimated_date: Option<NaiveDate>,
    pub status: ProjectionStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectionStatus {
    Reached,
    OnTrack,
    AwayFromTarget,
    FlatTrend,
    InsufficientData,
    NoCurrentWeight,
}

impl ProjectionStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Reached => "reached",
            Self::OnTrack => "on track",
            Self::AwayFromTarget => "away from target",
            Self::FlatTrend => "flat trend",
            Self::InsufficientData => "insufficient",
            Self::NoCurrentWeight => "no current weight",
        }
    }
}

#[derive(Debug)]
pub struct TrendAnalysis {
    pub reference_date: NaiveDate,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub total_records: usize,
    pub data_status: DataStatus,
    pub short_term_average: PeriodAverage,
    pub trend_kg_per_week: Option<f64>,
    pub trend_class: Option<TrendClass>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataStatus {
    NoData,
    Insufficient,
    Sufficient,
}

impl DataStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::NoData => "no data",
            Self::Insufficient => "insufficient",
            Self::Sufficient => "sufficient",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrendClass {
    LosingFast,
    LosingModerate,
    Stable,
    GainingModerate,
    GainingFast,
}

impl TrendClass {
    pub fn label(self) -> &'static str {
        match self {
            Self::LosingFast => "losing fast",
            Self::LosingModerate => "losing moderately",
            Self::Stable => "stable",
            Self::GainingModerate => "gaining moderately",
            Self::GainingFast => "gaining fast",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AdviceRecommendation {
    pub direction: &'static str,
    pub intensity: &'static str,
    pub action: &'static str,
    pub caution: bool,
}

pub fn comparison_range(reference_date: NaiveDate) -> (NaiveDate, NaiveDate) {
    (reference_date - Duration::days(365), reference_date)
}

pub fn advice_range(reference_date: NaiveDate) -> (NaiveDate, NaiveDate) {
    (
        reference_date - Duration::days(ADVICE_WINDOW_DAYS - 1),
        reference_date,
    )
}

pub fn calculate_bmi(weight_kg: f64) -> f64 {
    weight_kg / (HEIGHT_METERS * HEIGHT_METERS)
}

pub fn bmi_for_average(average_kg: Option<f64>) -> Option<f64> {
    average_kg.map(calculate_bmi)
}

pub fn classify_bmi(bmi: f64) -> BmiCategory {
    if bmi < 18.5 {
        BmiCategory::Underweight
    } else if bmi < 25.0 {
        BmiCategory::Normal
    } else if bmi < 30.0 {
        BmiCategory::Overweight
    } else {
        BmiCategory::Obesity
    }
}

pub fn compare_weights(records: &[WeightRecord], reference_date: NaiveDate) -> WeightComparison {
    let recent_end = reference_date;
    let recent_start = reference_date - Duration::days(RECENT_AVERAGE_DAYS - 1);
    let recent_average = period_average("recent 4 weeks", recent_start, recent_end, records);

    let recent_value = recent_average.average_kg;
    let points = [
        ("1 month  ago", reference_date - Duration::days(30)),
        ("3 months ago", reference_date - Duration::days(90)),
        ("6 months ago", reference_date - Duration::days(183)),
        ("1 year   ago", reference_date - Duration::days(365)),
    ]
    .into_iter()
    .map(|(label, target_date)| {
        let start = target_date - Duration::days(POINT_WINDOW_DAYS);
        let end = target_date + Duration::days(POINT_WINDOW_DAYS);
        let sample_count = count_between(records, start, end);
        let direct_average = average_between(records, start, end);
        let filled_average = direct_average
            .is_none()
            .then(|| interpolate_at(records, target_date))
            .flatten();
        let average = direct_average.or(filled_average);
        let value_source = match (sample_count, filled_average) {
            (count, _) if count > 0 => ComparisonValueSource::Direct,
            (0, Some(_)) => ComparisonValueSource::Filled,
            _ => ComparisonValueSource::Missing,
        };
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
            value_source,
        }
    })
    .collect();

    WeightComparison {
        recent_average,
        points,
    }
}

pub fn build_diet_advice(
    records: &[WeightRecord],
    reference_date: NaiveDate,
    goal: DietGoal,
) -> DietAdvice {
    let analysis = analyze_trend(records, reference_date);
    let (interpretation, recommendation) = match analysis.trend_class {
        Some(trend_class) if analysis.data_status == DataStatus::Sufficient => {
            advice_for(goal, trend_class)
        }
        _ => (
            "Recent records are not enough to support a diet adjustment.",
            None,
        ),
    };

    DietAdvice {
        goal,
        analysis,
        recommendation,
        interpretation,
    }
}

pub fn build_target_projection(
    records: &[WeightRecord],
    reference_date: NaiveDate,
    target_kg: f64,
) -> TargetProjection {
    let analysis = analyze_trend(records, reference_date);
    let current_average_kg = analysis.short_term_average.average_kg;
    let remaining_kg = current_average_kg.map(|current| current - target_kg);
    let (status, estimated_date) = match (current_average_kg, analysis.trend_kg_per_week) {
        (None, _) => (ProjectionStatus::NoCurrentWeight, None),
        (Some(current), _) if (current - target_kg).abs() <= 0.05 => {
            (ProjectionStatus::Reached, Some(reference_date))
        }
        (_, _) if analysis.data_status != DataStatus::Sufficient => {
            (ProjectionStatus::InsufficientData, None)
        }
        (Some(_), Some(trend)) if trend.abs() <= 0.05 => (ProjectionStatus::FlatTrend, None),
        (Some(current), Some(trend)) => {
            let weeks = (target_kg - current) / trend;
            if weeks <= 0.0 {
                (ProjectionStatus::AwayFromTarget, None)
            } else {
                let days = (weeks * 7.0).ceil() as i64;
                (
                    ProjectionStatus::OnTrack,
                    Some(reference_date + Duration::days(days)),
                )
            }
        }
        (Some(_), None) => (ProjectionStatus::InsufficientData, None),
    };

    TargetProjection {
        target_kg,
        analysis,
        current_average_kg,
        remaining_kg,
        estimated_date,
        status,
    }
}

pub fn analyze_trend(records: &[WeightRecord], reference_date: NaiveDate) -> TrendAnalysis {
    let (start, end) = advice_range(reference_date);
    let total_records = count_between(records, start, end);
    let recent_start = end - Duration::days(ADVICE_ENDPOINT_DAYS - 1);
    let short_term_average = period_average("recent 7 days", recent_start, end, records);
    let first_end = start + Duration::days(ADVICE_ENDPOINT_DAYS - 1);
    let last_start = end - Duration::days(ADVICE_ENDPOINT_DAYS - 1);
    let first_average = average_between(records, start, first_end);
    let last_average = average_between(records, last_start, end);
    let first_count = count_between(records, start, first_end);
    let last_count = count_between(records, last_start, end);
    let data_status = if total_records == 0 {
        DataStatus::NoData
    } else if total_records < MIN_ADVICE_RECORDS
        || first_count < MIN_ENDPOINT_RECORDS
        || last_count < MIN_ENDPOINT_RECORDS
    {
        DataStatus::Insufficient
    } else {
        DataStatus::Sufficient
    };

    let trend_kg_per_week = if data_status == DataStatus::Sufficient {
        first_average
            .zip(last_average)
            .map(|(first, last)| (last - first) / 3.0)
    } else {
        None
    };
    let trend_class = trend_kg_per_week.map(classify_trend);

    TrendAnalysis {
        reference_date,
        start,
        end,
        total_records,
        data_status,
        short_term_average,
        trend_kg_per_week,
        trend_class,
    }
}

fn classify_trend(kg_per_week: f64) -> TrendClass {
    if kg_per_week <= -0.9 {
        TrendClass::LosingFast
    } else if kg_per_week < -0.2 {
        TrendClass::LosingModerate
    } else if kg_per_week <= 0.2 {
        TrendClass::Stable
    } else if kg_per_week < 0.7 {
        TrendClass::GainingModerate
    } else {
        TrendClass::GainingFast
    }
}

fn advice_for(
    goal: DietGoal,
    trend_class: TrendClass,
) -> (&'static str, Option<AdviceRecommendation>) {
    match (goal, trend_class) {
        (_, TrendClass::LosingFast) => (
            "Weight is dropping quickly; prioritize caution before adding more restriction.",
            Some(AdviceRecommendation {
                direction: "increase or ease restriction",
                intensity: "cautious",
                action: "Add a small amount of regular food or reduce restriction, then observe for two weeks.",
                caution: true,
            }),
        ),
        (_, TrendClass::GainingFast) => (
            "Weight is rising quickly; treat this as a high-intensity signal and make only conservative changes.",
            Some(AdviceRecommendation {
                direction: "tighten intake carefully",
                intensity: "cautious",
                action: "Start with lower-risk changes such as reducing sweet drinks, late snacks, or oversized portions.",
                caution: true,
            }),
        ),
        (DietGoal::Cut, TrendClass::LosingModerate) => (
            "The current trend supports a fat loss goal.",
            Some(AdviceRecommendation {
                direction: "keep current diet direction",
                intensity: "steady",
                action: "Keep the current structure and avoid adding extra restriction while the trend is working.",
                caution: false,
            }),
        ),
        (DietGoal::Cut, TrendClass::Stable) => (
            "Weight is stable while the goal is fat loss.",
            Some(AdviceRecommendation {
                direction: "slightly reduce intake",
                intensity: "light",
                action: "Make one small change, such as reducing snacks, sweet drinks, or a small portion of staple foods.",
                caution: false,
            }),
        ),
        (DietGoal::Cut, TrendClass::GainingModerate) => (
            "Weight is trending up while the goal is fat loss.",
            Some(AdviceRecommendation {
                direction: "reduce intake",
                intensity: "moderate",
                action: "Tighten one or two repeatable habits, then reassess after two weeks of records.",
                caution: false,
            }),
        ),
        (DietGoal::Maintain, TrendClass::LosingModerate) => (
            "Weight is trending down while the goal is maintenance.",
            Some(AdviceRecommendation {
                direction: "slightly increase intake",
                intensity: "light",
                action: "Add a small consistent portion to regular meals and keep tracking the trend.",
                caution: false,
            }),
        ),
        (DietGoal::Maintain, TrendClass::Stable) => (
            "The current trend supports a maintenance goal.",
            Some(AdviceRecommendation {
                direction: "keep current diet direction",
                intensity: "steady",
                action: "Keep the current routine and continue monitoring weekly movement.",
                caution: false,
            }),
        ),
        (DietGoal::Maintain, TrendClass::GainingModerate) => (
            "Weight is trending up while the goal is maintenance.",
            Some(AdviceRecommendation {
                direction: "slightly reduce intake",
                intensity: "light",
                action: "Trim one repeatable source of extra intake and reassess after two weeks.",
                caution: false,
            }),
        ),
        (DietGoal::Gain, TrendClass::LosingModerate) => (
            "Weight is trending down while the goal is weight gain.",
            Some(AdviceRecommendation {
                direction: "increase intake",
                intensity: "moderate",
                action: "Add a consistent extra portion around meals or training days, then reassess the trend.",
                caution: false,
            }),
        ),
        (DietGoal::Gain, TrendClass::Stable) => (
            "Weight is stable while the goal is weight gain.",
            Some(AdviceRecommendation {
                direction: "slightly increase intake",
                intensity: "light",
                action: "Add one small repeatable portion each day and keep tracking.",
                caution: false,
            }),
        ),
        (DietGoal::Gain, TrendClass::GainingModerate) => (
            "The current trend supports a weight gain goal.",
            Some(AdviceRecommendation {
                direction: "keep current diet direction",
                intensity: "steady",
                action: "Keep the current structure and monitor that the rate does not accelerate.",
                caution: false,
            }),
        ),
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

fn interpolate_at(records: &[WeightRecord], target_date: NaiveDate) -> Option<f64> {
    let before = records
        .iter()
        .filter(|record| record.record_date < target_date)
        .max_by_key(|record| record.record_date)?;
    let after = records
        .iter()
        .filter(|record| record.record_date > target_date)
        .min_by_key(|record| record.record_date)?;
    let total_days = (after.record_date - before.record_date).num_days() as f64;
    if total_days <= 0.0 {
        return None;
    }

    let target_days = (target_date - before.record_date).num_days() as f64;
    let fraction = target_days / total_days;

    Some(before.weight_kg + ((after.weight_kg - before.weight_kg) * fraction))
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
    fn calculates_bmi_from_fixed_height() {
        let bmi = calculate_bmi(70.0);

        assert!((bmi - 23.388_686).abs() < 0.000_001);
        assert_eq!(format!("{bmi:.2}"), "23.39");
    }

    #[test]
    fn classifies_bmi_boundary_values() {
        assert_eq!(classify_bmi(18.49), BmiCategory::Underweight);
        assert_eq!(classify_bmi(18.5), BmiCategory::Normal);
        assert_eq!(classify_bmi(24.99), BmiCategory::Normal);
        assert_eq!(classify_bmi(25.0), BmiCategory::Overweight);
        assert_eq!(classify_bmi(29.99), BmiCategory::Overweight);
        assert_eq!(classify_bmi(30.0), BmiCategory::Obesity);
    }

    #[test]
    fn maps_optional_average_to_optional_bmi() {
        assert_eq!(bmi_for_average(None), None);
        assert!((bmi_for_average(Some(70.0)).unwrap() - 23.388_686).abs() < 0.000_001);
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
        assert_eq!(
            comparison.points[0].value_source,
            ComparisonValueSource::Direct
        );
        assert_eq!(comparison.points[0].delta_from_recent_kg, Some(-3.0));
    }

    #[test]
    fn fills_empty_historical_window_from_surrounding_records() {
        let reference_date = NaiveDate::from_ymd_opt(2026, 5, 14).unwrap();
        let records = vec![
            record("2026-05-01", 80.0),
            record("2026-05-14", 78.0),
            record("2026-04-01", 82.0),
            record("2026-04-27", 80.0),
        ];

        let comparison = compare_weights(&records, reference_date);
        let point = &comparison.points[0];

        assert_eq!(point.sample_count, 0);
        assert_eq!(point.value_source, ComparisonValueSource::Filled);
        assert_eq!(point.average_kg, Some(81.0));
        assert!((point.delta_from_recent_kg.unwrap() - -1.6667).abs() < 0.001);
    }

    #[test]
    fn leaves_historical_window_missing_without_surrounding_records() {
        let reference_date = NaiveDate::from_ymd_opt(2026, 5, 14).unwrap();
        let records = vec![
            record("2026-05-01", 80.0),
            record("2026-05-14", 78.0),
            record("2026-04-27", 80.0),
        ];

        let comparison = compare_weights(&records, reference_date);
        let point = &comparison.points[0];

        assert_eq!(point.sample_count, 0);
        assert_eq!(point.value_source, ComparisonValueSource::Missing);
        assert_eq!(point.average_kg, None);
        assert_eq!(point.delta_from_recent_kg, None);
    }

    #[test]
    fn does_not_fill_missing_recent_baseline() {
        let reference_date = NaiveDate::from_ymd_opt(2026, 5, 14).unwrap();
        let records = vec![record("2026-04-14", 82.0)];

        let comparison = compare_weights(&records, reference_date);

        assert_eq!(comparison.recent_average.average_kg, None);
        assert_eq!(
            comparison.points[0].value_source,
            ComparisonValueSource::Direct
        );
        assert_eq!(comparison.points[0].average_kg, Some(82.0));
        assert_eq!(comparison.points[0].delta_from_recent_kg, None);
    }

    #[test]
    fn uses_one_year_fetch_range() {
        let reference_date = NaiveDate::from_ymd_opt(2026, 5, 14).unwrap();

        assert_eq!(
            comparison_range(reference_date),
            (
                NaiveDate::from_ymd_opt(2025, 5, 14).unwrap(),
                reference_date
            )
        );
    }

    #[test]
    fn calculates_smoothed_medium_term_trend() {
        let reference_date = NaiveDate::from_ymd_opt(2026, 5, 28).unwrap();
        let records = vec![
            record("2026-05-01", 80.0),
            record("2026-05-02", 80.2),
            record("2026-05-03", 80.1),
            record("2026-05-04", 80.0),
            record("2026-05-12", 79.0),
            record("2026-05-13", 78.9),
            record("2026-05-14", 79.1),
            record("2026-05-22", 78.5),
            record("2026-05-23", 78.6),
            record("2026-05-24", 78.5),
            record("2026-05-25", 78.6),
        ];

        let analysis = analyze_trend(&records, reference_date);

        assert_eq!(analysis.data_status, DataStatus::Sufficient);
        assert_eq!(analysis.trend_class, Some(TrendClass::LosingModerate));
        assert_eq!(analysis.short_term_average.sample_count, 4);
        assert!(
            (analysis.trend_kg_per_week.unwrap() - -0.5).abs() < 0.05,
            "expected about -0.5 kg/week"
        );
    }

    #[test]
    fn marks_sparse_trend_data_as_insufficient() {
        let reference_date = NaiveDate::from_ymd_opt(2026, 5, 28).unwrap();
        let records = vec![
            record("2026-05-01", 80.0),
            record("2026-05-14", 79.0),
            record("2026-05-28", 78.0),
        ];

        let analysis = analyze_trend(&records, reference_date);

        assert_eq!(analysis.data_status, DataStatus::Insufficient);
        assert_eq!(analysis.trend_kg_per_week, None);
        assert_eq!(analysis.trend_class, None);
    }

    #[test]
    fn suppresses_advice_when_data_is_insufficient() {
        let reference_date = NaiveDate::from_ymd_opt(2026, 5, 28).unwrap();
        let records = vec![record("2026-05-28", 78.0)];

        let advice = build_diet_advice(&records, reference_date, DietGoal::Cut);

        assert_eq!(advice.analysis.data_status, DataStatus::Insufficient);
        assert_eq!(advice.recommendation, None);
    }

    #[test]
    fn maps_goal_specific_advice_rules() {
        let reference_date = NaiveDate::from_ymd_opt(2026, 5, 28).unwrap();
        let records = vec![
            record("2026-05-01", 80.0),
            record("2026-05-02", 80.1),
            record("2026-05-03", 80.0),
            record("2026-05-04", 80.2),
            record("2026-05-12", 80.6),
            record("2026-05-13", 80.8),
            record("2026-05-14", 80.7),
            record("2026-05-22", 81.2),
            record("2026-05-23", 81.3),
            record("2026-05-24", 81.2),
            record("2026-05-25", 81.4),
        ];

        let cut = build_diet_advice(&records, reference_date, DietGoal::Cut);
        let gain = build_diet_advice(&records, reference_date, DietGoal::Gain);

        assert_eq!(cut.analysis.trend_class, Some(TrendClass::GainingModerate));
        assert_eq!(cut.recommendation.unwrap().direction, "reduce intake");
        assert_eq!(
            gain.recommendation.unwrap().direction,
            "keep current diet direction"
        );
    }

    #[test]
    fn projects_target_date_when_trend_moves_toward_goal() {
        let reference_date = NaiveDate::from_ymd_opt(2026, 5, 28).unwrap();
        let records = vec![
            record("2026-05-01", 74.0),
            record("2026-05-02", 74.1),
            record("2026-05-03", 74.0),
            record("2026-05-04", 73.9),
            record("2026-05-12", 73.0),
            record("2026-05-13", 72.9),
            record("2026-05-14", 73.1),
            record("2026-05-22", 72.5),
            record("2026-05-23", 72.4),
            record("2026-05-24", 72.5),
            record("2026-05-25", 72.6),
        ];

        let projection = build_target_projection(&records, reference_date, 70.0);

        assert_eq!(projection.status, ProjectionStatus::OnTrack);
        assert_eq!(
            projection.estimated_date,
            Some(NaiveDate::from_ymd_opt(2026, 7, 2).unwrap())
        );
    }

    #[test]
    fn does_not_project_when_trend_moves_away_from_goal() {
        let reference_date = NaiveDate::from_ymd_opt(2026, 5, 28).unwrap();
        let records = vec![
            record("2026-05-01", 72.0),
            record("2026-05-02", 72.1),
            record("2026-05-03", 72.0),
            record("2026-05-04", 72.2),
            record("2026-05-12", 72.6),
            record("2026-05-13", 72.8),
            record("2026-05-14", 72.7),
            record("2026-05-22", 73.2),
            record("2026-05-23", 73.3),
            record("2026-05-24", 73.2),
            record("2026-05-25", 73.4),
        ];

        let projection = build_target_projection(&records, reference_date, 70.0);

        assert_eq!(projection.status, ProjectionStatus::AwayFromTarget);
        assert_eq!(projection.estimated_date, None);
    }
}
