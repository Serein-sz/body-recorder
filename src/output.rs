use crate::models::WeightRecord;
use crate::stats::{
    AdviceRecommendation, ComparisonPoint, DataStatus, DietAdvice, PeriodAverage, TrendAnalysis,
};
use crate::use_cases::{AddWeightResult, DeleteWeightResult, UpdateWeightResult};
use ansi_term::ANSIString;
use ansi_term::Colour::{Green, Red, Yellow};
use ansi_term::Style;
use chrono::NaiveDate;
use std::fmt::Write;

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
        writeln!(output, "{} {:.2} kg", record.record_date, record.weight_kg).unwrap();
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
        "{:<18} {:>10} {:>8}  range",
        "period", "average", "records"
    )
    .unwrap();
    writeln!(output, "{}", "-".repeat(62)).unwrap();
    writeln!(
        output,
        "{:<18} {:>10} {:>8}  {} to {}",
        recent.label,
        format_average(recent.average_kg),
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
        "{:<14} {:<10} {:>10} {:>10} {:>8}  {:<10} range",
        "period", "target", "average", "delta", "records", "status"
    )
    .unwrap();
    writeln!(output, "{}", "-".repeat(98)).unwrap();

    for point in points {
        let delta = paint_delta(point.delta_from_recent_kg);
        let status = paint_status(point.delta_from_recent_kg);
        writeln!(
            output,
            "{:<14} {:<10} {:>10} {} {:>8}  {} {} to {}",
            point.label,
            point.target_date,
            format_average(point.average_kg),
            delta,
            point.sample_count,
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
        "{:<16} {} from {} record(s)",
        "7-day average",
        format_average(analysis.short_term_average.average_kg),
        analysis.short_term_average.sample_count
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

fn format_trend(value: Option<f64>) -> String {
    value
        .map(|trend| format!("{trend:+.2} kg/week"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn paint_data_status(value: DataStatus) -> ANSIString<'static> {
    let text = value.label().to_string();

    match value {
        DataStatus::Sufficient => Green.paint(text),
        DataStatus::Insufficient => Yellow.paint(text),
        DataStatus::NoData => Red.paint(text),
    }
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
    use crate::stats::{DataStatus, DietGoal, PeriodAverage, TrendAnalysis};

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

        assert_eq!(output, "2026-05-14 72.46 kg\n");
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
        assert!(output.contains("direction: no adjustment recommendation"));
        assert!(output.contains("reason: record at least 10 weights across the 28-day window"));
        assert!(output.contains("note: this is trend-based guidance, not medical advice."));
    }
}
