use crate::models::WeightRecord;
use crate::stats::{
    AdviceRecommendation, BmiCategory, ComparisonPoint, DataStatus, DietAdvice, PeriodAverage,
    ProjectionStatus, TargetProjection, TrendAnalysis, bmi_for_average, calculate_bmi,
    classify_bmi,
};
use crate::use_cases::{AddWeightResult, DeleteWeightResult, UpdateWeightResult};
use ansi_term::ANSIString;
use ansi_term::Colour::{Green, Red, Yellow};
use ansi_term::Style;
use chrono::NaiveDate;
use std::fmt::Write;

const BMI_VALUE_WIDTH: usize = 9;
const BMI_CATEGORY_WIDTH: usize = 11;
const BMI_CELL_WIDTH: usize = BMI_VALUE_WIDTH + 1 + BMI_CATEGORY_WIDTH;

pub fn render_add_weight(result: &AddWeightResult) -> String {
    format!(
        "saved {} {:.2} kg\n",
        result.record.record_date, result.record.weight_kg
    )
}

pub fn render_list_weights(records: &[WeightRecord]) -> String {
    if records.is_empty() {
        return "no weight records found\n".to_string();
    }

    let mut output = String::new();
    for record in records {
        writeln!(
            output,
            "{} {:.2} kg  {}",
            record.record_date,
            record.weight_kg,
            format_bmi(Some(calculate_bmi(record.weight_kg)))
        )
        .unwrap();
    }
    output
}

pub fn render_update_weight(result: &UpdateWeightResult) -> String {
    match &result.record {
        Some(record) => format!(
            "updated {} {:.2} kg\n",
            record.record_date, record.weight_kg
        ),
        None => format!("no record found for {}\n", result.date),
    }
}

pub fn render_delete_weight(result: &DeleteWeightResult) -> String {
    if result.deleted {
        format!("deleted {}\n", result.date)
    } else {
        format!("no record found for {}\n", result.date)
    }
}

pub fn render_comparison(
    reference_date: NaiveDate,
    total_records: usize,
    recent: &PeriodAverage,
    points: &[ComparisonPoint],
) -> String {
    let mut output = String::new();
    let title = Style::new().bold().paint("Weight comparison");
    writeln!(output, "{title}").unwrap();
    writeln!(
        output,
        "reference date: {reference_date}   records loaded: {total_records}"
    )
    .unwrap();
    writeln!(output).unwrap();

    writeln!(output, "{}", Style::new().bold().paint("Baseline")).unwrap();
    writeln!(
        output,
        "{:<18} {:>10} {:<width$} {:>8}  range",
        "period",
        "average",
        "BMI",
        "records",
        width = BMI_CELL_WIDTH
    )
    .unwrap();
    writeln!(output, "{}", "-".repeat(86)).unwrap();
    writeln!(
        output,
        "{:<18} {:>10} {} {:>8}  {} to {}",
        recent.label,
        format_average(recent.average_kg),
        format_bmi_cell(bmi_for_average(recent.average_kg)),
        recent.sample_count,
        recent.start,
        recent.end
    )
    .unwrap();
    writeln!(output).unwrap();

    writeln!(
        output,
        "{}",
        Style::new().bold().paint("Compared with baseline")
    )
    .unwrap();
    writeln!(
        output,
        "{:<14} {:<10} {:>10} {:<width$} {:>10} {:>8}  {:<8} {:<10} range",
        "period",
        "target",
        "average",
        "BMI",
        "delta",
        "records",
        "source",
        "status",
        width = BMI_CELL_WIDTH
    )
    .unwrap();
    writeln!(output, "{}", "-".repeat(130)).unwrap();

    for point in points {
        let delta = paint_delta(point.delta_from_recent_kg);
        let status = paint_status(point.delta_from_recent_kg);
        writeln!(
            output,
            "{:<14} {:<10} {:>10} {} {} {:>8}  {:<8} {} {} to {}",
            point.label,
            point.target_date,
            format_average(point.average_kg),
            format_bmi_cell(bmi_for_average(point.average_kg)),
            delta,
            point.sample_count,
            point.value_source.label(),
            status,
            point.start,
            point.end
        )
        .unwrap();
    }

    output
}

pub fn render_advice(advice: &DietAdvice) -> String {
    let mut output = String::new();
    let title = Style::new().bold().paint("Diet advice");
    writeln!(output, "{title}").unwrap();
    writeln!(
        output,
        "reference date: {}   goal: {}   records loaded: {}",
        advice.analysis.reference_date,
        advice.goal.label(),
        advice.analysis.total_records
    )
    .unwrap();
    writeln!(output).unwrap();

    render_trend_summary(&mut output, &advice.analysis);
    writeln!(output).unwrap();

    writeln!(output, "{}", Style::new().bold().paint("Interpretation")).unwrap();
    writeln!(output, "{}", advice.interpretation).unwrap();
    writeln!(output).unwrap();

    writeln!(output, "{}", Style::new().bold().paint("Diet adjustment")).unwrap();
    match &advice.recommendation {
        Some(recommendation) => render_recommendation(&mut output, recommendation),
        None => {
            writeln!(output, "direction: no adjustment recommendation").unwrap();
            match advice.analysis.data_status {
                DataStatus::NoData => {
                    writeln!(output, "reason: no usable recent records were found").unwrap();
                }
                DataStatus::Insufficient => {
                    writeln!(
                        output,
                        "reason: record at least 10 weights across the 28-day window, including several near the start and end"
                    )
                    .unwrap();
                }
                DataStatus::Sufficient => {
                    writeln!(output, "reason: trend signal is unavailable").unwrap();
                }
            }
        }
    }
    writeln!(
        output,
        "note: this is trend-based guidance, not medical advice."
    )
    .unwrap();

    output
}

pub fn render_target_projection(projection: &TargetProjection) -> String {
    let mut output = String::new();
    let title = Style::new().bold().paint("Target estimate");
    writeln!(output, "{title}").unwrap();
    writeln!(
        output,
        "reference date: {}   target: {:.2} kg   records loaded: {}",
        projection.analysis.reference_date, projection.target_kg, projection.analysis.total_records
    )
    .unwrap();
    writeln!(output).unwrap();

    render_trend_summary(&mut output, &projection.analysis);
    writeln!(output).unwrap();

    writeln!(output, "{}", Style::new().bold().paint("Projection")).unwrap();
    writeln!(
        output,
        "{:<16} {}",
        "current avg",
        format_average(projection.current_average_kg)
    )
    .unwrap();
    writeln!(
        output,
        "{:<16} {}",
        "remaining",
        format_remaining(projection.remaining_kg)
    )
    .unwrap();
    writeln!(
        output,
        "{:<16} {}",
        "status",
        paint_projection_status(projection.status)
    )
    .unwrap();
    writeln!(
        output,
        "{:<16} {}",
        "estimate",
        format_estimated_date(projection.estimated_date)
    )
    .unwrap();
    writeln!(
        output,
        "note: this is a simple 4-week trend estimate, not a precise prediction."
    )
    .unwrap();

    output
}

fn render_trend_summary(output: &mut String, analysis: &TrendAnalysis) {
    writeln!(output, "{}", Style::new().bold().paint("Trend")).unwrap();
    writeln!(
        output,
        "{:<16} {} to {}",
        "analysis range", analysis.start, analysis.end
    )
    .unwrap();
    writeln!(
        output,
        "{:<16} {}",
        "data status",
        paint_data_status(analysis.data_status)
    )
    .unwrap();
    writeln!(
        output,
        "{:<16} {}",
        "28-day trend",
        format_trend(analysis.trend_kg_per_week)
    )
    .unwrap();
    writeln!(
        output,
        "{:<16} {}",
        "trend class",
        analysis
            .trend_class
            .map(|class| class.label().to_string())
            .unwrap_or_else(|| "n/a".to_string())
    )
    .unwrap();
    writeln!(
        output,
        "{:<16} {} from {} record(s)  {}",
        "7-day average",
        format_average(analysis.short_term_average.average_kg),
        analysis.short_term_average.sample_count,
        format_bmi(bmi_for_average(analysis.short_term_average.average_kg))
    )
    .unwrap();
}

fn render_recommendation(output: &mut String, recommendation: &AdviceRecommendation) {
    writeln!(output, "direction: {}", recommendation.direction).unwrap();
    writeln!(output, "intensity: {}", recommendation.intensity).unwrap();
    writeln!(output, "action: {}", recommendation.action).unwrap();
    if recommendation.caution {
        writeln!(
            output,
            "caution: change slowly and reassess after two weeks of records"
        )
        .unwrap();
    }
}

fn format_average(value: Option<f64>) -> String {
    value
        .map(|average| format!("{average:.2} kg"))
        .unwrap_or_else(|| "no data".to_string())
}

fn format_bmi(value: Option<f64>) -> ANSIString<'static> {
    match value {
        Some(bmi) => {
            let category = classify_bmi(bmi);
            Style::new().paint(format!("BMI {bmi:.2} {}", paint_bmi_category(category)))
        }
        None => Style::new().paint("BMI n/a".to_string()),
    }
}

fn format_bmi_cell(value: Option<f64>) -> String {
    match value {
        Some(bmi) => {
            let category = classify_bmi(bmi);
            format!(
                "{:<BMI_VALUE_WIDTH$} {}",
                format!("BMI {bmi:.2}"),
                paint_bmi_category_padded(category)
            )
        }
        None => format!("{:<BMI_CELL_WIDTH$}", "BMI n/a"),
    }
}

fn format_trend(value: Option<f64>) -> String {
    value
        .map(|trend| format!("{trend:+.2} kg/week"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn paint_bmi_category(category: BmiCategory) -> ANSIString<'static> {
    let text = category.label().to_string();

    match category {
        BmiCategory::Normal => Green.paint(text),
        BmiCategory::Underweight | BmiCategory::Overweight => Yellow.paint(text),
        BmiCategory::Obesity => Red.paint(text),
    }
}

fn paint_bmi_category_padded(category: BmiCategory) -> ANSIString<'static> {
    let text = format!("{:<BMI_CATEGORY_WIDTH$}", category.label());

    match category {
        BmiCategory::Normal => Green.paint(text),
        BmiCategory::Underweight | BmiCategory::Overweight => Yellow.paint(text),
        BmiCategory::Obesity => Red.paint(text),
    }
}

fn paint_data_status(value: DataStatus) -> ANSIString<'static> {
    let text = value.label().to_string();

    match value {
        DataStatus::Sufficient => Green.paint(text),
        DataStatus::Insufficient => Yellow.paint(text),
        DataStatus::NoData => Red.paint(text),
    }
}

fn paint_projection_status(value: ProjectionStatus) -> ANSIString<'static> {
    let text = value.label().to_string();

    match value {
        ProjectionStatus::Reached | ProjectionStatus::OnTrack => Green.paint(text),
        ProjectionStatus::FlatTrend | ProjectionStatus::InsufficientData => Yellow.paint(text),
        ProjectionStatus::AwayFromTarget | ProjectionStatus::NoCurrentWeight => Red.paint(text),
    }
}

fn format_remaining(value: Option<f64>) -> String {
    value
        .map(|remaining| {
            if remaining > 0.05 {
                format!("{remaining:.2} kg above target")
            } else if remaining < -0.05 {
                format!("{:.2} kg below target", remaining.abs())
            } else {
                "at target".to_string()
            }
        })
        .unwrap_or_else(|| "n/a".to_string())
}

fn format_estimated_date(value: Option<NaiveDate>) -> String {
    value
        .map(|date| date.to_string())
        .unwrap_or_else(|| "n/a".to_string())
}

fn paint_delta(value: Option<f64>) -> ANSIString<'static> {
    let text = format!("{:>10}", format_delta(value));

    match value {
        Some(delta) if delta < -0.05 => Green.paint(text),
        Some(delta) if delta > 0.05 => Red.paint(text),
        Some(_) => Yellow.paint(text),
        None => Style::new().paint(text),
    }
}

fn format_delta(value: Option<f64>) -> String {
    value
        .map(|delta| format!("{delta:+.2} kg"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn paint_status(value: Option<f64>) -> ANSIString<'static> {
    let text = format!("{:<10}", format_status(value));

    match value {
        Some(delta) if delta < -0.05 => Green.paint(text),
        Some(delta) if delta > 0.05 => Red.paint(text),
        Some(_) => Yellow.paint(text),
        None => Style::new().paint(text),
    }
}

fn format_status(value: Option<f64>) -> &'static str {
    match value {
        Some(delta) if delta < -0.05 => "lower",
        Some(delta) if delta > 0.05 => "higher",
        Some(_) => "steady",
        None => "missing",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::{
        ComparisonValueSource, DataStatus, DietGoal, PeriodAverage, ProjectionStatus,
        TargetProjection, TrendAnalysis,
    };

    fn date(value: &str) -> NaiveDate {
        NaiveDate::parse_from_str(value, "%Y-%m-%d").unwrap()
    }

    fn record(value: &str, weight_kg: f64) -> WeightRecord {
        WeightRecord {
            record_date: date(value),
            weight_kg,
            created_at: None,
            updated_at: None,
        }
    }

    #[test]
    fn renders_empty_list_message() {
        assert_eq!(render_list_weights(&[]), "no weight records found\n");
    }

    #[test]
    fn renders_weight_records_in_existing_text_shape() {
        let output = render_list_weights(&[record("2026-05-14", 72.456)]);

        assert!(output.contains("2026-05-14 72.46 kg  BMI 24.21"));
        assert!(output.contains("normal"));
        assert!(output.contains("\u{1b}[32mnormal\u{1b}[0m"));
    }

    #[test]
    fn renders_update_missing_message() {
        let result = UpdateWeightResult {
            date: date("2026-05-14"),
            record: None,
        };

        assert_eq!(
            render_update_weight(&result),
            "no record found for 2026-05-14\n"
        );
    }

    #[test]
    fn renders_comparison_value_source() {
        let recent = PeriodAverage {
            label: "recent 4 weeks",
            start: date("2026-04-17"),
            end: date("2026-05-14"),
            average_kg: Some(79.0),
            sample_count: 2,
        };
        let points = vec![ComparisonPoint {
            label: "1 month  ago",
            target_date: date("2026-04-14"),
            start: date("2026-04-07"),
            end: date("2026-04-21"),
            average_kg: Some(81.0),
            sample_count: 0,
            delta_from_recent_kg: Some(-2.0),
            value_source: ComparisonValueSource::Filled,
        }];

        let output = render_comparison(date("2026-05-14"), 4, &recent, &points);

        assert!(output.contains("source"));
        assert!(output.contains("filled"));
        assert!(output.contains("BMI"));
        assert!(output.contains("27.06"));
        assert!(output.contains("overweight"));
    }

    #[test]
    fn renders_comparison_columns_aligned_with_colored_bmi_categories() {
        let recent = PeriodAverage {
            label: "recent 4 weeks",
            start: date("2026-04-17"),
            end: date("2026-05-14"),
            average_kg: Some(72.0),
            sample_count: 2,
        };
        let points = vec![
            ComparisonPoint {
                label: "1 month  ago",
                target_date: date("2026-04-14"),
                start: date("2026-04-07"),
                end: date("2026-04-21"),
                average_kg: Some(54.0),
                sample_count: 1,
                delta_from_recent_kg: Some(18.0),
                value_source: ComparisonValueSource::Direct,
            },
            ComparisonPoint {
                label: "3 months ago",
                target_date: date("2026-02-14"),
                start: date("2026-02-07"),
                end: date("2026-02-21"),
                average_kg: Some(72.0),
                sample_count: 1,
                delta_from_recent_kg: Some(0.0),
                value_source: ComparisonValueSource::Filled,
            },
            ComparisonPoint {
                label: "6 months ago",
                target_date: date("2025-11-12"),
                start: date("2025-11-05"),
                end: date("2025-11-19"),
                average_kg: Some(90.0),
                sample_count: 1,
                delta_from_recent_kg: Some(-18.0),
                value_source: ComparisonValueSource::Direct,
            },
        ];

        let output = strip_ansi(&render_comparison(date("2026-05-14"), 4, &recent, &points));
        let header = output
            .lines()
            .find(|line| line.contains("period") && line.contains("source"))
            .unwrap();
        let source_col = header.find("source").unwrap();

        for (label, source) in [
            ("1 month  ago", "direct"),
            ("3 months ago", "filled"),
            ("6 months ago", "direct"),
        ] {
            let line = output.lines().find(|line| line.contains(label)).unwrap();
            assert_eq!(line.find(source), Some(source_col), "{line}");
        }
    }

    #[test]
    fn renders_missing_comparison_bmi_as_unavailable() {
        let recent = PeriodAverage {
            label: "recent 4 weeks",
            start: date("2026-04-17"),
            end: date("2026-05-14"),
            average_kg: None,
            sample_count: 0,
        };
        let points = vec![ComparisonPoint {
            label: "1 month  ago",
            target_date: date("2026-04-14"),
            start: date("2026-04-07"),
            end: date("2026-04-21"),
            average_kg: None,
            sample_count: 0,
            delta_from_recent_kg: None,
            value_source: ComparisonValueSource::Missing,
        }];

        let output = render_comparison(date("2026-05-14"), 0, &recent, &points);

        assert!(output.contains("BMI n/a"));
        assert!(!output.contains("underweight"));
        assert!(!output.contains("normal"));
        assert!(!output.contains("overweight"));
        assert!(!output.contains("obesity"));
    }

    #[test]
    fn renders_advice_insufficient_data_reason() {
        let advice = DietAdvice {
            goal: DietGoal::Cut,
            interpretation: "Recent records are not enough to support a diet adjustment.",
            recommendation: None,
            analysis: TrendAnalysis {
                reference_date: date("2026-05-28"),
                start: date("2026-05-01"),
                end: date("2026-05-28"),
                total_records: 1,
                data_status: DataStatus::Insufficient,
                short_term_average: PeriodAverage {
                    label: "recent 7 days",
                    start: date("2026-05-22"),
                    end: date("2026-05-28"),
                    average_kg: Some(72.0),
                    sample_count: 1,
                },
                trend_kg_per_week: None,
                trend_class: None,
            },
        };

        let output = render_advice(&advice);

        assert!(output.contains("Diet advice"));
        assert!(output.contains("7-day average"));
        assert!(output.contains("BMI 24.06"));
        assert!(output.contains("normal"));
        assert!(output.contains("direction: no adjustment recommendation"));
        assert!(output.contains("reason: record at least 10 weights across the 28-day window"));
        assert!(output.contains("note: this is trend-based guidance, not medical advice."));
    }

    #[test]
    fn renders_target_projection() {
        let projection = TargetProjection {
            target_kg: 70.0,
            current_average_kg: Some(72.5),
            remaining_kg: Some(2.5),
            estimated_date: Some(date("2026-07-03")),
            status: ProjectionStatus::OnTrack,
            analysis: TrendAnalysis {
                reference_date: date("2026-05-28"),
                start: date("2026-05-01"),
                end: date("2026-05-28"),
                total_records: 11,
                data_status: DataStatus::Sufficient,
                short_term_average: PeriodAverage {
                    label: "recent 7 days",
                    start: date("2026-05-22"),
                    end: date("2026-05-28"),
                    average_kg: Some(72.5),
                    sample_count: 4,
                },
                trend_kg_per_week: Some(-0.5),
                trend_class: None,
            },
        };

        let output = render_target_projection(&projection);

        assert!(output.contains("Target estimate"));
        assert!(output.contains("target: 70.00 kg"));
        assert!(output.contains("current avg"));
        assert!(output.contains("2.50 kg above target"));
        assert!(output.contains("2026-07-03"));
        assert!(output.contains("simple 4-week trend estimate"));
    }

    #[test]
    fn colors_bmi_category_labels_by_status() {
        assert_eq!(
            paint_bmi_category(BmiCategory::Normal).to_string(),
            "\u{1b}[32mnormal\u{1b}[0m"
        );
        assert_eq!(
            paint_bmi_category(BmiCategory::Underweight).to_string(),
            "\u{1b}[33munderweight\u{1b}[0m"
        );
        assert_eq!(
            paint_bmi_category(BmiCategory::Overweight).to_string(),
            "\u{1b}[33moverweight\u{1b}[0m"
        );
        assert_eq!(
            paint_bmi_category(BmiCategory::Obesity).to_string(),
            "\u{1b}[31mobesity\u{1b}[0m"
        );
    }

    fn strip_ansi(value: &str) -> String {
        let mut output = String::new();
        let mut chars = value.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\u{1b}' && chars.peek() == Some(&'[') {
                chars.next();
                for escaped in chars.by_ref() {
                    if escaped.is_ascii_alphabetic() {
                        break;
                    }
                }
            } else {
                output.push(ch);
            }
        }

        output
    }
}
