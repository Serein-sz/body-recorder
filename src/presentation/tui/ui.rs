use super::{AnalysisView, App, InputField, LoadState, Mode, OperationStatus};
use crate::domain::stats::{
    AdviceRecommendation, BmiCategory, ComparisonPoint, ComparisonValueSource, DataStatus,
    PeriodAverage, ProjectionStatus, TdeeDataStatus, TrendAnalysis, TrendClass, analyze_trend,
    bmi_for_average, calculate_bmi, classify_bmi,
};
use crate::presentation::tui::app::advice_goal_label;
use chrono::NaiveDate;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};

const RECORDS_WIDTH_PERCENT: u16 = 44;
const ANALYSIS_WIDTH_PERCENT: u16 = 56;
const BMI_VALUE_WIDTH: usize = 10;
const BMI_CATEGORY_WIDTH: usize = 11;
const BMI_CELL_WIDTH: usize = BMI_VALUE_WIDTH + BMI_CATEGORY_WIDTH + 1;

pub(crate) fn render(frame: &mut Frame<'_>, app: &App) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(4),
        ])
        .split(frame.area());

    render_header(frame, app, root[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(RECORDS_WIDTH_PERCENT),
            Constraint::Percentage(ANALYSIS_WIDTH_PERCENT),
        ])
        .split(root[1]);

    render_records(frame, app, body[0]);
    render_analysis(frame, app, body[1]);
    render_status(frame, app, root[2]);
    render_overlay(frame, app);
}

fn render_header(frame: &mut Frame<'_>, app: &App, area: ratatui::layout::Rect) {
    let mode = match &app.mode {
        Mode::Normal => "normal",
        Mode::Adding(_) => "add",
        Mode::Editing(_) => "edit",
        Mode::ConfirmDelete => "delete",
    };
    frame.render_widget(
        Paragraph::new(header_line(app, mode)).block(panel("br tui")),
        area,
    );
}

fn render_records(frame: &mut Frame<'_>, app: &App, area: ratatui::layout::Rect) {
    let lines = match &app.status {
        OperationStatus::Loading if app.records.is_empty() => {
            vec![Line::styled("loading records...", loading_style())]
        }
        OperationStatus::Error(message) if app.records.is_empty() => {
            vec![Line::styled(format!("error: {message}"), error_style())]
        }
        _ if app.records.is_empty() => {
            vec![Line::styled("no weight records found", unavailable_style())]
        }
        _ => app
            .records
            .iter()
            .enumerate()
            .map(|(index, record)| {
                let marker = if index == app.selected { ">" } else { " " };
                let style = if index == app.selected {
                    selected_row_style()
                } else {
                    Style::default()
                };
                let bmi = calculate_bmi(record.weight_kg);
                Line::from(vec![
                    Span::styled(
                        format!(
                            "{marker} {}  {:.2} kg  ",
                            record.record_date, record.weight_kg
                        ),
                        style,
                    ),
                    Span::styled(format!("BMI {bmi:.2} "), style),
                    Span::styled(
                        classify_bmi(bmi).label().to_string(),
                        style.patch(bmi_category_style(classify_bmi(bmi))),
                    ),
                ])
            })
            .collect(),
    };

    frame.render_widget(
        Paragraph::new(lines)
            .block(panel("Recent records"))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn render_analysis(frame: &mut Frame<'_>, app: &App, area: ratatui::layout::Rect) {
    let title = format!("Analysis - {}", app.active_view.label());
    let lines = match app.active_view {
        AnalysisView::Summary => summary_lines(app),
        AnalysisView::Compare => compare_lines(app),
        AnalysisView::Advice => advice_lines(app),
        AnalysisView::Target => target_lines(app),
    };

    frame.render_widget(
        Paragraph::new(lines)
            .block(panel(&title))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn summary_lines(app: &App) -> Vec<Line<'static>> {
    let analysis = analyze_trend(&app.records, app.reference_date);
    let mut lines = vec![
        Line::from(format!("range: {} to {}", analysis.start, analysis.end)),
        Line::from(format!("data: {}", analysis.data_status.label())),
        trend_line(analysis.trend_kg_per_week, analysis.trend_class),
        short_term_average_line(&analysis),
        Line::from(""),
    ];
    lines.extend(tdee_summary_lines(app));
    lines
}

fn compare_lines(app: &App) -> Vec<Line<'static>> {
    match &app.compare {
        LoadState::NotLoaded => vec![Line::styled(
            "Press Tab to open Compare or r to load it.",
            unavailable_style(),
        )],
        LoadState::Loading => vec![Line::styled("loading compare analysis...", loading_style())],
        LoadState::Error(message) => {
            vec![Line::styled(
                format!("compare error: {message}"),
                error_style(),
            )]
        }
        LoadState::Ready(result) => {
            let mut lines = vec![
                Line::from(format!(
                    "ref: {}   records loaded: {}",
                    result.reference_date, result.total_records
                )),
                Line::from(format!(
                    "baseline: {}   {}   {}   {} record(s)",
                    result.comparison.recent_average.label,
                    format_average(&result.comparison.recent_average),
                    format_bmi(bmi_for_average(result.comparison.recent_average.average_kg)),
                    result.comparison.recent_average.sample_count
                )),
                Line::from(""),
                Line::styled(
                    "period        average      BMI                    delta       source",
                    title_style(),
                ),
            ];
            lines.extend(result.comparison.points.iter().map(compare_point_line));
            lines
        }
    }
}

fn compare_point_line(point: &ComparisonPoint) -> Line<'static> {
    let mut spans = vec![
        Span::raw(format!("{:<13} ", point.label)),
        Span::styled(
            format!("{:<12} ", format_optional_kg(point.average_kg)),
            optional_value_style(point.average_kg),
        ),
    ];
    spans.extend(bmi_spans(bmi_for_average(point.average_kg), BMI_CELL_WIDTH));
    spans.extend([
        Span::styled(
            format!("{:<12} ", format_delta(point.delta_from_recent_kg)),
            delta_style(point.delta_from_recent_kg),
        ),
        Span::styled(point.value_source.label(), source_style(point.value_source)),
    ]);
    Line::from(spans)
}

fn advice_lines(app: &App) -> Vec<Line<'static>> {
    match &app.advice {
        LoadState::NotLoaded => vec![Line::styled(
            "Press Tab to open Advice or r to load it.",
            unavailable_style(),
        )],
        LoadState::Loading => vec![Line::styled("loading advice analysis...", loading_style())],
        LoadState::Error(message) => {
            vec![Line::styled(
                format!("advice error: {message}"),
                error_style(),
            )]
        }
        LoadState::Ready(result) => {
            let advice = &result.advice;
            let analysis = &advice.analysis;
            let mut lines = vec![
                Line::from(vec![
                    Span::raw(format!(
                        "goal: {}   data: ",
                        advice_goal_label(app.advice_goal)
                    )),
                    Span::styled(
                        analysis.data_status.label(),
                        data_status_style(analysis.data_status),
                    ),
                ]),
                trend_line(analysis.trend_kg_per_week, analysis.trend_class),
                short_term_average_line(analysis),
                Line::from(""),
                Line::styled("Interpretation", title_style()),
                Line::from(advice.interpretation.to_string()),
                Line::from(""),
                Line::styled("Diet adjustment", title_style()),
            ];

            match &advice.recommendation {
                Some(recommendation) => lines.extend(recommendation_lines(recommendation)),
                None => lines.push(Line::styled(
                    "no adjustment recommendation",
                    unavailable_style(),
                )),
            }

            lines
        }
    }
}

fn recommendation_lines(recommendation: &AdviceRecommendation) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(format!("direction: {}", recommendation.direction)),
        Line::from(vec![
            Span::raw("intensity: "),
            Span::styled(
                recommendation.intensity,
                recommendation_intensity_style(recommendation.intensity),
            ),
        ]),
        Line::from(format!("action: {}", recommendation.action)),
    ];
    if recommendation.caution {
        lines.push(Line::styled(
            "caution: change slowly and reassess",
            caution_style(),
        ));
    }
    lines
}

fn target_lines(app: &App) -> Vec<Line<'static>> {
    match &app.target {
        LoadState::NotLoaded => vec![Line::styled(
            "Press Tab to open Target or r to load it.",
            unavailable_style(),
        )],
        LoadState::Loading => vec![Line::styled("loading target estimate...", loading_style())],
        LoadState::Error(message) => {
            vec![Line::styled(
                format!("target error: {message}"),
                error_style(),
            )]
        }
        LoadState::Ready(result) => {
            let projection = &result.projection;
            let analysis = &projection.analysis;
            vec![
                Line::from(format!(
                    "target: {:.2} kg   data: {}",
                    projection.target_kg,
                    analysis.data_status.label()
                )),
                trend_line(analysis.trend_kg_per_week, analysis.trend_class),
                short_term_average_line(analysis),
                Line::from(""),
                Line::styled("Projection", title_style()),
                Line::from(format!(
                    "current avg: {}",
                    format_optional_kg(projection.current_average_kg)
                )),
                Line::from(format!(
                    "remaining: {}",
                    format_remaining(projection.remaining_kg)
                )),
                Line::from(vec![
                    Span::raw("status: "),
                    Span::styled(
                        projection.status.label(),
                        projection_status_style(projection.status),
                    ),
                ]),
                Line::from(format!(
                    "estimate: {}",
                    format_estimated_date(projection.estimated_date)
                )),
                Line::styled(
                    "simple 4-week trend estimate, not a precise prediction",
                    unavailable_style(),
                ),
            ]
        }
    }
}

fn tdee_summary_lines(app: &App) -> Vec<Line<'static>> {
    match &app.tdee {
        LoadState::NotLoaded => vec![Line::styled(
            "TDEE estimate: press r to load summary data.",
            unavailable_style(),
        )],
        LoadState::Loading => vec![Line::styled("TDEE estimate: loading...", loading_style())],
        LoadState::Error(message) => {
            vec![Line::styled(
                format!("TDEE error: {message}"),
                error_style(),
            )]
        }
        LoadState::Ready(result) => {
            let estimate = &result.estimate;
            let mut lines = vec![
                Line::styled("TDEE estimate", title_style()),
                Line::from(vec![
                    Span::raw("data: "),
                    Span::styled(
                        estimate.data_status.label(),
                        tdee_data_status_style(estimate.data_status),
                    ),
                    Span::raw(format!("   samples: {}", estimate.sample_count)),
                ]),
            ];

            if let Some(tdee_kcal) = estimate.tdee_kcal {
                lines.extend([
                    Line::from(format!(
                        "estimate: {:.0} kcal/day   7-day avg: {}",
                        tdee_kcal,
                        format_optional_kg(estimate.average_weight_kg)
                    )),
                    Line::from(format!(
                        "basis: {} {}y {:.0}cm activity {:.2}",
                        estimate.basis.sex.label(),
                        estimate.basis.age_years,
                        estimate.basis.height_cm,
                        estimate.basis.activity_factor
                    )),
                ]);
            } else {
                lines.push(Line::styled(
                    "no recent weight data available",
                    unavailable_style(),
                ));
            }
            lines.push(Line::styled(
                "formula-based estimate, not a precise prescription",
                unavailable_style(),
            ));

            lines
        }
    }
}

fn render_status(frame: &mut Frame<'_>, app: &App, area: ratatui::layout::Rect) {
    let help = match &app.mode {
        Mode::Normal if app.active_view == AnalysisView::Advice => {
            "[tab] view  [g] goal  [r] refresh  [a/e/d] records  [q] quit"
        }
        Mode::Normal => "[tab] view  [r] refresh  [a/e/d] records  [q] quit",
        Mode::Adding(_) | Mode::Editing(_) => "[tab] field  [enter] save  [esc] cancel",
        Mode::ConfirmDelete => "[enter] delete  [esc] cancel",
    };

    frame.render_widget(
        Paragraph::new(vec![
            status_line(&app.status),
            Line::styled(help, inactive_style()),
        ])
        .block(panel("Status")),
        area,
    );
}

fn render_overlay(frame: &mut Frame<'_>, app: &App) {
    match &app.mode {
        Mode::Adding(input) => render_input(frame, "Add weight", input, true),
        Mode::Editing(input) => render_input(frame, "Edit weight", input, false),
        Mode::ConfirmDelete => {
            let area = centered_rect(frame.area(), 58, 28);
            let selected = app
                .records
                .get(app.selected)
                .map(|record| format!("{} {:.2} kg", record.record_date, record.weight_kg))
                .unwrap_or_else(|| "no record selected".to_string());
            frame.render_widget(Clear, area);
            frame.render_widget(
                Paragraph::new(vec![
                    Line::from("Delete selected record?"),
                    Line::from(selected),
                    Line::from("Press Enter to delete or Esc to cancel."),
                ])
                .block(panel("Confirm delete"))
                .wrap(Wrap { trim: false }),
                area,
            );
        }
        Mode::Normal => {}
    }
}

fn render_input(
    frame: &mut Frame<'_>,
    title: &str,
    input: &super::app::InputState,
    allow_date: bool,
) {
    let area = centered_rect(frame.area(), 62, 32);
    let date_marker = if input.field == InputField::Date {
        ">"
    } else {
        " "
    };
    let weight_marker = if input.field == InputField::Weight {
        ">"
    } else {
        " "
    };
    let date_value = if input.date.is_empty() {
        "today".to_string()
    } else {
        input.date.clone()
    };
    let mut lines = Vec::new();
    if allow_date {
        lines.push(Line::from(format!("{date_marker} date: {date_value}")));
    } else {
        lines.push(Line::from(format!("  date: {date_value}")));
    }
    lines.push(Line::from(format!(
        "{weight_marker} weight: {}",
        input.weight
    )));
    lines.push(Line::from("Enter saves. Esc cancels."));

    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(lines)
            .block(panel(title))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn panel(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style())
        .title(Span::styled(title.to_string(), title_style()))
}

fn header_line(app: &App, mode: &str) -> Line<'static> {
    let selected = if app.records.is_empty() {
        "-".to_string()
    } else {
        (app.selected + 1).to_string()
    };
    let mut spans = vec![Span::raw(format!(
        "Body Recorder   records: {}   selected: {}   mode: {}   views: ",
        app.records.len(),
        selected,
        mode
    ))];
    spans.extend(view_tab_spans(app.active_view));
    Line::from(spans)
}

fn view_tab_spans(active: AnalysisView) -> Vec<Span<'static>> {
    [
        AnalysisView::Summary,
        AnalysisView::Compare,
        AnalysisView::Advice,
        AnalysisView::Target,
    ]
    .into_iter()
    .enumerate()
    .flat_map(|(index, view)| {
        let mut spans = Vec::new();
        if index > 0 {
            spans.push(Span::raw(" "));
        }
        if view == active {
            spans.push(Span::styled(
                format!("[{}]", view.label()),
                active_tab_style(),
            ));
        } else {
            spans.push(Span::styled(view.label().to_string(), inactive_style()));
        }
        spans
    })
    .collect()
}

fn trend_line(value: Option<f64>, class: Option<TrendClass>) -> Line<'static> {
    let trend = value
        .map(|value| format!("{value:+.2} kg/week"))
        .unwrap_or_else(|| "n/a".to_string());
    let class_label = class
        .map(|class| class.label().to_string())
        .unwrap_or_else(|| "n/a".to_string());

    Line::from(vec![
        Span::raw("trend: "),
        Span::styled(trend, optional_value_style(value)),
        Span::raw("   class: "),
        Span::styled(class_label, trend_class_style(class)),
    ])
}

fn short_term_average_line(analysis: &TrendAnalysis) -> Line<'static> {
    let mut spans = vec![
        Span::raw("7-day average: "),
        Span::styled(
            format_optional_kg(analysis.short_term_average.average_kg),
            optional_value_style(analysis.short_term_average.average_kg),
        ),
        Span::raw(format!(
            " from {} record(s)   ",
            analysis.short_term_average.sample_count
        )),
    ];
    spans.extend(bmi_spans(
        bmi_for_average(analysis.short_term_average.average_kg),
        0,
    ));
    Line::from(spans)
}

fn status_line(status: &OperationStatus) -> Line<'static> {
    match status {
        OperationStatus::Idle => Line::styled("ready", message_style()),
        OperationStatus::Loading => Line::styled("loading records...", loading_style()),
        OperationStatus::Saving(message) => Line::styled(*message, loading_style()),
        OperationStatus::Error(message) => Line::styled(format!("error: {message}"), error_style()),
        OperationStatus::Message(message) => Line::styled(message.clone(), message_style()),
    }
}

fn border_style() -> Style {
    Style::default().fg(Color::DarkGray)
}

fn title_style() -> Style {
    Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD)
}

fn active_tab_style() -> Style {
    Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD)
}

fn selected_row_style() -> Style {
    active_tab_style()
}

fn inactive_style() -> Style {
    Style::default().fg(Color::Gray)
}

fn loading_style() -> Style {
    Style::default().fg(Color::Yellow)
}

fn message_style() -> Style {
    Style::default().fg(Color::Green)
}

fn error_style() -> Style {
    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
}

fn unavailable_style() -> Style {
    Style::default().fg(Color::DarkGray)
}

fn caution_style() -> Style {
    Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD)
}

fn favorable_style() -> Style {
    Style::default().fg(Color::Green)
}

fn unfavorable_style() -> Style {
    Style::default().fg(Color::Red)
}

fn neutral_style() -> Style {
    Style::default().fg(Color::Yellow)
}

fn optional_value_style(value: Option<f64>) -> Style {
    if value.is_some() {
        Style::default()
    } else {
        unavailable_style()
    }
}

fn delta_style(value: Option<f64>) -> Style {
    match value {
        Some(delta) if delta < -0.05 => favorable_style(),
        Some(delta) if delta > 0.05 => unfavorable_style(),
        Some(_) => neutral_style(),
        None => unavailable_style(),
    }
}

fn source_style(source: ComparisonValueSource) -> Style {
    match source {
        ComparisonValueSource::Direct => favorable_style(),
        ComparisonValueSource::Filled => neutral_style(),
        ComparisonValueSource::Missing => unavailable_style(),
    }
}

fn data_status_style(status: DataStatus) -> Style {
    match status {
        DataStatus::Sufficient => favorable_style(),
        DataStatus::Insufficient => caution_style(),
        DataStatus::NoData => error_style(),
    }
}

fn tdee_data_status_style(status: TdeeDataStatus) -> Style {
    match status {
        TdeeDataStatus::Normal => favorable_style(),
        TdeeDataStatus::LowSample => caution_style(),
        TdeeDataStatus::NoData => error_style(),
    }
}

fn projection_status_style(status: ProjectionStatus) -> Style {
    match status {
        ProjectionStatus::Reached | ProjectionStatus::OnTrack => favorable_style(),
        ProjectionStatus::FlatTrend | ProjectionStatus::InsufficientData => caution_style(),
        ProjectionStatus::AwayFromTarget | ProjectionStatus::NoCurrentWeight => error_style(),
    }
}

fn trend_class_style(class: Option<TrendClass>) -> Style {
    match class {
        Some(TrendClass::Stable) => favorable_style(),
        Some(TrendClass::LosingFast | TrendClass::GainingFast) => caution_style(),
        Some(TrendClass::LosingModerate | TrendClass::GainingModerate) => neutral_style(),
        None => unavailable_style(),
    }
}

fn bmi_category_style(category: BmiCategory) -> Style {
    match category {
        BmiCategory::Normal => favorable_style(),
        BmiCategory::Underweight | BmiCategory::Overweight => neutral_style(),
        BmiCategory::Obesity => unfavorable_style(),
    }
}

fn recommendation_intensity_style(intensity: &str) -> Style {
    match intensity {
        "steady" => favorable_style(),
        "light" => neutral_style(),
        "moderate" | "cautious" => caution_style(),
        _ => Style::default(),
    }
}

fn format_average(period: &PeriodAverage) -> String {
    format_optional_kg(period.average_kg)
}

fn format_bmi(value: Option<f64>) -> String {
    value
        .map(|value| {
            let category = classify_bmi(value);
            format!("BMI {value:.2} {}", category.label())
        })
        .unwrap_or_else(|| "BMI n/a".to_string())
}

fn bmi_spans(value: Option<f64>, width: usize) -> Vec<Span<'static>> {
    match value {
        Some(value) => {
            let category = classify_bmi(value);
            let value_text = format!("BMI {value:.2}");
            let visual_width = BMI_VALUE_WIDTH + BMI_CATEGORY_WIDTH;
            let trailing_width = width.saturating_sub(visual_width);
            vec![
                Span::raw(format!("{value_text:<BMI_VALUE_WIDTH$}")),
                Span::styled(
                    format!("{:<BMI_CATEGORY_WIDTH$}", category.label()),
                    bmi_category_style(category),
                ),
                Span::raw(" ".repeat(trailing_width)),
            ]
        }
        None => vec![Span::styled(
            pad_right("BMI n/a".to_string(), width),
            unavailable_style(),
        )],
    }
}

fn pad_right(value: String, width: usize) -> String {
    if width == 0 {
        value
    } else {
        format!("{value:<width$}")
    }
}

fn format_optional_kg(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.2} kg"))
        .unwrap_or_else(|| "no data".to_string())
}

fn format_delta(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:+.2} kg"))
        .unwrap_or_else(|| "n/a".to_string())
}

fn format_remaining(value: Option<f64>) -> String {
    value
        .map(|remaining| {
            if remaining > 0.05 {
                format!("{remaining:.2} kg above")
            } else if remaining < -0.05 {
                format!("{:.2} kg below", remaining.abs())
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

fn centered_rect(
    area: ratatui::layout::Rect,
    percent_x: u16,
    percent_y: u16,
) -> ratatui::layout::Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

#[cfg(test)]
pub(crate) fn render_to_text(app: &App, width: u16, height: u16) -> String {
    let backend = ratatui::backend::TestBackend::new(width, height);
    let mut terminal = ratatui::Terminal::new(backend).unwrap();
    terminal.draw(|frame| render(frame, app)).unwrap();
    format!("{:?}", terminal.backend().buffer())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::use_cases::{AdviceResult, CompareWeightsResult, TargetResult, TdeeResult};
    use crate::domain::goals::AdviceGoal;
    use crate::domain::models::WeightRecord;
    use crate::domain::stats::{
        BmiCategory, ComparisonValueSource, DataStatus, DietGoal, ProjectionStatus, TdeeDataStatus,
        TrendClass, build_diet_advice, build_target_projection, build_tdee_estimate,
        compare_weights,
    };
    use chrono::NaiveDate;

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

    #[test]
    fn renders_loaded_records() {
        let mut app = App::new_with_date(date("2026-05-19"));
        app.records = vec![record(date("2026-05-19"), 72.4)];

        let output = render_to_text(&app, 100, 28);

        assert!(output.contains("Recent records"));
        assert!(output.contains("2026-05-19"));
        assert!(output.contains("72.40 kg"));
        assert!(output.contains("BMI 24.19 normal"));
    }

    #[test]
    fn renders_empty_state() {
        let app = App::new_with_date(date("2026-05-19"));

        let output = render_to_text(&app, 100, 28);

        assert!(output.contains("no weight records found"));
    }

    #[test]
    fn renders_loading_state() {
        let mut app = App::new_with_date(date("2026-05-19"));
        app.status = OperationStatus::Loading;

        let output = render_to_text(&app, 100, 28);

        assert!(output.contains("loading records"));
    }

    #[test]
    fn renders_error_state() {
        let mut app = App::new_with_date(date("2026-05-19"));
        app.status = OperationStatus::Error("storage unavailable".to_string());

        let output = render_to_text(&app, 100, 28);

        assert!(output.contains("storage unavailable"));
    }

    #[test]
    fn renders_main_panels_with_rounded_borders() {
        let app = App::new_with_date(date("2026-05-19"));

        let output = render_to_text(&app, 100, 28);

        assert!(output.contains("╭"));
        assert!(output.contains("╮"));
        assert!(output.contains("╰"));
        assert!(output.contains("╯"));
    }

    #[test]
    fn renders_overlay_with_rounded_borders() {
        let mut app = App::new_with_date(date("2026-05-19"));
        app.records = vec![record(date("2026-05-19"), 72.4)];
        app.mode = Mode::ConfirmDelete;

        let output = render_to_text(&app, 100, 28);

        assert!(output.contains("Confirm delete"));
        assert!(output.contains("╭"));
        assert!(output.contains("╮"));
        assert!(output.contains("╰"));
        assert!(output.contains("╯"));
    }

    #[test]
    fn analysis_panel_is_wider_than_records_panel() {
        const {
            assert!(RECORDS_WIDTH_PERCENT < ANALYSIS_WIDTH_PERCENT);
        }
    }

    #[test]
    fn renders_analysis_view_tabs() {
        let mut app = App::new_with_date(date("2026-05-19"));
        app.active_view = AnalysisView::Compare;

        let output = render_to_text(&app, 120, 28);

        assert!(output.contains("Summary [Compare] Advice Target"));
        assert!(!output.contains("TDEE]"));
    }

    #[test]
    fn styles_panel_titles_and_active_tabs() {
        assert_eq!(border_style().fg, Some(Color::DarkGray));
        assert_eq!(title_style().fg, Some(Color::Cyan));
        assert!(title_style().add_modifier.contains(Modifier::BOLD));

        let spans = view_tab_spans(AnalysisView::Compare);
        let compare = spans
            .iter()
            .find(|span| span.content.as_ref() == "[Compare]")
            .unwrap();

        assert_eq!(compare.style.fg, Some(Color::Cyan));
        assert!(compare.style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn styles_record_selection_and_operation_statuses() {
        assert_eq!(selected_row_style().fg, Some(Color::Cyan));
        assert!(selected_row_style().add_modifier.contains(Modifier::BOLD));

        assert_eq!(
            status_line(&OperationStatus::Loading).style.fg,
            Some(Color::Yellow)
        );
        assert_eq!(
            status_line(&OperationStatus::Message("saved".to_string()))
                .style
                .fg,
            Some(Color::Green)
        );
        assert_eq!(
            status_line(&OperationStatus::Error("failed".to_string()))
                .style
                .fg,
            Some(Color::Red)
        );
    }

    #[test]
    fn styles_compare_delta_and_source_semantics() {
        assert_eq!(delta_style(Some(-0.5)).fg, Some(Color::Green));
        assert_eq!(delta_style(Some(0.5)).fg, Some(Color::Red));
        assert_eq!(delta_style(Some(0.0)).fg, Some(Color::Yellow));
        assert_eq!(delta_style(None).fg, Some(Color::DarkGray));

        assert_eq!(
            source_style(ComparisonValueSource::Direct).fg,
            Some(Color::Green)
        );
        assert_eq!(
            source_style(ComparisonValueSource::Filled).fg,
            Some(Color::Yellow)
        );
        assert_eq!(
            source_style(ComparisonValueSource::Missing).fg,
            Some(Color::DarkGray)
        );
    }

    #[test]
    fn styles_advice_status_trend_and_recommendation_semantics() {
        assert_eq!(
            data_status_style(DataStatus::Sufficient).fg,
            Some(Color::Green)
        );
        assert_eq!(
            data_status_style(DataStatus::Insufficient).fg,
            Some(Color::Yellow)
        );
        assert_eq!(
            trend_class_style(Some(TrendClass::Stable)).fg,
            Some(Color::Green)
        );
        assert_eq!(
            trend_class_style(Some(TrendClass::GainingFast)).fg,
            Some(Color::Yellow)
        );
        assert_eq!(
            recommendation_intensity_style("steady").fg,
            Some(Color::Green)
        );
        assert_eq!(
            recommendation_intensity_style("moderate").fg,
            Some(Color::Yellow)
        );
        assert!(caution_style().add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn styles_projection_status_semantics() {
        assert_eq!(
            projection_status_style(ProjectionStatus::OnTrack).fg,
            Some(Color::Green)
        );
        assert_eq!(
            projection_status_style(ProjectionStatus::FlatTrend).fg,
            Some(Color::Yellow)
        );
        assert_eq!(
            projection_status_style(ProjectionStatus::AwayFromTarget).fg,
            Some(Color::Red)
        );
    }

    #[test]
    fn styles_tdee_data_status_semantics() {
        assert_eq!(
            tdee_data_status_style(TdeeDataStatus::Normal).fg,
            Some(Color::Green)
        );
        assert_eq!(
            tdee_data_status_style(TdeeDataStatus::LowSample).fg,
            Some(Color::Yellow)
        );
        assert_eq!(
            tdee_data_status_style(TdeeDataStatus::NoData).fg,
            Some(Color::Red)
        );
    }

    #[test]
    fn styles_bmi_categories() {
        assert_eq!(
            bmi_category_style(BmiCategory::Normal).fg,
            Some(Color::Green)
        );
        assert_eq!(
            bmi_category_style(BmiCategory::Underweight).fg,
            Some(Color::Yellow)
        );
        assert_eq!(
            bmi_category_style(BmiCategory::Overweight).fg,
            Some(Color::Yellow)
        );
        assert_eq!(
            bmi_category_style(BmiCategory::Obesity).fg,
            Some(Color::Red)
        );
    }

    #[test]
    fn builds_fixed_width_bmi_spans_for_compare_rows() {
        let underweight = bmi_spans(Some(18.0), BMI_CELL_WIDTH);
        let normal = bmi_spans(Some(23.0), BMI_CELL_WIDTH);
        let overweight = bmi_spans(Some(27.0), BMI_CELL_WIDTH);

        for spans in [&underweight, &normal, &overweight] {
            let visible_width = spans
                .iter()
                .map(|span| span.content.chars().count())
                .sum::<usize>();
            assert_eq!(visible_width, BMI_CELL_WIDTH);
        }

        assert_eq!(underweight[1].content, "underweight");
        assert_eq!(normal[1].content, "normal     ");
        assert_eq!(overweight[1].content, "overweight ");
        assert_eq!(normal[1].style.fg, Some(Color::Green));
        assert_eq!(overweight[1].style.fg, Some(Color::Yellow));
    }

    #[test]
    fn builds_fixed_width_missing_bmi_span_for_compare_rows() {
        let spans = bmi_spans(None, BMI_CELL_WIDTH);

        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].content.chars().count(), BMI_CELL_WIDTH);
        assert_eq!(spans[0].style.fg, Some(Color::DarkGray));
    }

    #[test]
    fn renders_compare_loaded_state() {
        let reference_date = date("2026-05-19");
        let records = vec![
            record(reference_date, 72.0),
            record(date("2026-04-19"), 73.0),
            record(date("2025-11-17"), 75.0),
            record(date("2025-05-19"), 78.0),
        ];
        let mut app = App::new_with_date(reference_date);
        app.active_view = AnalysisView::Compare;
        app.compare = LoadState::Ready(CompareWeightsResult {
            reference_date,
            total_records: records.len(),
            comparison: compare_weights(&records, reference_date),
        });

        let output = render_to_text(&app, 120, 32);

        assert!(output.contains("Analysis - Compare"));
        assert!(output.contains("baseline"));
        assert!(output.contains("period"));
        assert!(output.contains("BMI"));
        assert!(output.contains("normal"));
        assert!(output.contains("direct"));
    }

    #[test]
    fn renders_compare_loading_and_error_states() {
        let mut app = App::new_with_date(date("2026-05-19"));
        app.active_view = AnalysisView::Compare;
        app.compare = LoadState::Loading;

        let loading = render_to_text(&app, 120, 28);
        assert!(loading.contains("loading compare analysis"));

        app.compare = LoadState::Error("compare failed".to_string());
        let error = render_to_text(&app, 120, 28);
        assert!(error.contains("compare failed"));
    }

    #[test]
    fn renders_advice_loaded_state() {
        let reference_date = date("2026-05-19");
        let records = advice_records(reference_date);
        let mut app = App::new_with_date(reference_date);
        app.active_view = AnalysisView::Advice;
        app.advice_goal = AdviceGoal::Cut;
        app.advice = LoadState::Ready(AdviceResult {
            advice: build_diet_advice(&records, reference_date, DietGoal::Cut),
        });

        let output = render_to_text(&app, 120, 34);

        assert!(output.contains("Analysis - Advice"));
        assert!(output.contains("goal: fat loss"));
        assert!(output.contains("7-day average"));
        assert!(output.contains("BMI"));
        assert!(output.contains("normal"));
        assert!(output.contains("Interpretation"));
        assert!(output.contains("Diet adjustment"));
    }

    #[test]
    fn renders_advice_insufficient_loading_and_error_states() {
        let reference_date = date("2026-05-19");
        let records = vec![record(reference_date, 72.0)];
        let mut app = App::new_with_date(reference_date);
        app.active_view = AnalysisView::Advice;
        app.advice = LoadState::Ready(AdviceResult {
            advice: build_diet_advice(&records, reference_date, DietGoal::Cut),
        });

        let insufficient = render_to_text(&app, 120, 34);
        assert!(insufficient.contains("data: insufficient"));
        assert!(insufficient.contains("no adjustment recommendation"));

        app.advice = LoadState::Loading;
        let loading = render_to_text(&app, 120, 28);
        assert!(loading.contains("loading advice analysis"));

        app.advice = LoadState::Error("advice failed".to_string());
        let error = render_to_text(&app, 120, 28);
        assert!(error.contains("advice failed"));
    }

    #[test]
    fn renders_target_loaded_state() {
        let reference_date = date("2026-05-28");
        let records = advice_records(reference_date);
        let mut app = App::new_with_date(reference_date);
        app.active_view = AnalysisView::Target;
        app.target = LoadState::Ready(TargetResult {
            projection: build_target_projection(&records, reference_date, 70.0),
        });

        let output = render_to_text(&app, 120, 34);

        assert!(output.contains("Analysis - Target"));
        assert!(output.contains("target: 70.00 kg"));
        assert!(output.contains("Projection"));
        assert!(output.contains("current avg"));
        assert!(output.contains("remaining"));
        assert!(output.contains("estimate"));
    }

    #[test]
    fn renders_target_loading_and_error_states() {
        let mut app = App::new_with_date(date("2026-05-19"));
        app.active_view = AnalysisView::Target;
        app.target = LoadState::Loading;

        let loading = render_to_text(&app, 120, 28);
        assert!(loading.contains("loading target estimate"));

        app.target = LoadState::Error("target failed".to_string());
        let error = render_to_text(&app, 120, 28);
        assert!(error.contains("target failed"));
    }

    #[test]
    fn renders_tdee_loaded_state() {
        let reference_date = date("2026-05-25");
        let records = vec![
            record(date("2026-05-19"), 70.0),
            record(date("2026-05-21"), 71.0),
            record(reference_date, 72.0),
        ];
        let mut app = App::new_with_date(reference_date);
        app.tdee = LoadState::Ready(TdeeResult {
            estimate: build_tdee_estimate(&records, reference_date),
        });

        let output = render_to_text(&app, 120, 34);

        assert!(output.contains("Analysis - Summary"));
        assert!(output.contains("TDEE estimate"));
        assert!(output.contains("estimate: 2674 kcal/day"));
        assert!(output.contains("7-day avg: 71.00 kg"));
        assert!(output.contains("samples: 3"));
        assert!(output.contains("male"));
        assert!(output.contains("activity 1.60"));
        assert!(output.contains("formula-based estimate"));
    }

    #[test]
    fn renders_tdee_low_sample_no_data_loading_and_error_states() {
        let reference_date = date("2026-05-25");
        let mut app = App::new_with_date(reference_date);
        app.tdee = LoadState::Ready(TdeeResult {
            estimate: build_tdee_estimate(&[record(reference_date, 72.0)], reference_date),
        });

        let low_sample = render_to_text(&app, 120, 34);
        assert!(low_sample.contains("data: low sample"));
        assert!(low_sample.contains("estimate:"));

        app.tdee = LoadState::Ready(TdeeResult {
            estimate: build_tdee_estimate(&[], reference_date),
        });
        let no_data = render_to_text(&app, 120, 34);
        assert!(no_data.contains("data: no data"));
        assert!(no_data.contains("no recent weight data available"));

        app.tdee = LoadState::Loading;
        let loading = render_to_text(&app, 120, 28);
        assert!(loading.contains("TDEE estimate: loading"));

        app.tdee = LoadState::Error("TDEE failed".to_string());
        let error = render_to_text(&app, 120, 28);
        assert!(error.contains("TDEE failed"));
    }

    fn advice_records(reference_date: NaiveDate) -> Vec<WeightRecord> {
        (0..28)
            .map(|offset| {
                let date = reference_date - chrono::Duration::days(offset);
                let weight = 72.0 + (offset as f64 * 0.03);
                record(date, weight)
            })
            .collect()
    }
}
