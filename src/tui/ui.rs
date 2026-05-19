use super::{AnalysisView, App, InputField, LoadState, Mode, OperationStatus};
use crate::stats::{AdviceRecommendation, ComparisonPoint, PeriodAverage};
use crate::tui::app::advice_goal_label;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};

const RECORDS_WIDTH_PERCENT: u16 = 40;
const ANALYSIS_WIDTH_PERCENT: u16 = 60;

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
    let text = format!(
        "Body Recorder   records: {}   selected: {}   mode: {}   views: {}",
        app.records.len(),
        if app.records.is_empty() {
            "-".to_string()
        } else {
            (app.selected + 1).to_string()
        },
        mode,
        view_tabs(app.active_view)
    );
    frame.render_widget(Paragraph::new(text).block(panel("br tui")), area);
}

fn render_records(frame: &mut Frame<'_>, app: &App, area: ratatui::layout::Rect) {
    let lines = match &app.status {
        OperationStatus::Loading if app.records.is_empty() => {
            vec![Line::from("loading records...")]
        }
        OperationStatus::Error(message) if app.records.is_empty() => {
            vec![Line::from(format!("error: {message}"))]
        }
        _ if app.records.is_empty() => vec![Line::from("no weight records found")],
        _ => app
            .records
            .iter()
            .enumerate()
            .map(|(index, record)| {
                let marker = if index == app.selected { ">" } else { " " };
                let style = if index == app.selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                Line::from(vec![Span::styled(
                    format!(
                        "{marker} {}  {:.2} kg",
                        record.record_date, record.weight_kg
                    ),
                    style,
                )])
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
    };

    frame.render_widget(
        Paragraph::new(lines)
            .block(panel(&title))
            .wrap(Wrap { trim: false }),
        area,
    );
}

fn summary_lines(app: &App) -> Vec<Line<'static>> {
    app.trend_lines().into_iter().map(Line::from).collect()
}

fn compare_lines(app: &App) -> Vec<Line<'static>> {
    match &app.compare {
        LoadState::NotLoaded => vec![Line::from("Press ] to open Compare or r to load it.")],
        LoadState::Loading => vec![Line::from("loading compare analysis...")],
        LoadState::Error(message) => vec![Line::from(format!("compare error: {message}"))],
        LoadState::Ready(result) => {
            let mut lines = vec![
                Line::from(format!(
                    "ref: {}   records loaded: {}",
                    result.reference_date, result.total_records
                )),
                Line::from(format!(
                    "baseline: {}   {}   {} record(s)",
                    result.comparison.recent_average.label,
                    format_average(&result.comparison.recent_average),
                    result.comparison.recent_average.sample_count
                )),
                Line::from(""),
                Line::from("period        average      delta       source"),
            ];
            lines.extend(result.comparison.points.iter().map(compare_point_line));
            lines
        }
    }
}

fn compare_point_line(point: &ComparisonPoint) -> Line<'static> {
    Line::from(format!(
        "{:<13} {:<12} {:<11} {}",
        point.label,
        format_optional_kg(point.average_kg),
        format_delta(point.delta_from_recent_kg),
        point.value_source.label()
    ))
}

fn advice_lines(app: &App) -> Vec<Line<'static>> {
    match &app.advice {
        LoadState::NotLoaded => vec![Line::from("Press ] to open Advice or r to load it.")],
        LoadState::Loading => vec![Line::from("loading advice analysis...")],
        LoadState::Error(message) => vec![Line::from(format!("advice error: {message}"))],
        LoadState::Ready(result) => {
            let advice = &result.advice;
            let analysis = &advice.analysis;
            let mut lines = vec![
                Line::from(format!(
                    "goal: {}   data: {}",
                    advice_goal_label(app.advice_goal),
                    analysis.data_status.label()
                )),
                Line::from(format!(
                    "trend: {}   class: {}",
                    analysis
                        .trend_kg_per_week
                        .map(|value| format!("{value:+.2} kg/week"))
                        .unwrap_or_else(|| "n/a".to_string()),
                    analysis
                        .trend_class
                        .map(|class| class.label().to_string())
                        .unwrap_or_else(|| "n/a".to_string())
                )),
                Line::from(""),
                Line::from("Interpretation"),
                Line::from(advice.interpretation.to_string()),
                Line::from(""),
                Line::from("Diet adjustment"),
            ];

            match &advice.recommendation {
                Some(recommendation) => lines.extend(recommendation_lines(recommendation)),
                None => lines.push(Line::from("no adjustment recommendation")),
            }

            lines
        }
    }
}

fn recommendation_lines(recommendation: &AdviceRecommendation) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(format!("direction: {}", recommendation.direction)),
        Line::from(format!("intensity: {}", recommendation.intensity)),
        Line::from(format!("action: {}", recommendation.action)),
    ];
    if recommendation.caution {
        lines.push(Line::from("caution: change slowly and reassess"));
    }
    lines
}

fn render_status(frame: &mut Frame<'_>, app: &App, area: ratatui::layout::Rect) {
    let status = match &app.status {
        OperationStatus::Idle => "ready".to_string(),
        OperationStatus::Loading => "loading records...".to_string(),
        OperationStatus::Saving(message) => (*message).to_string(),
        OperationStatus::Error(message) => format!("error: {message}"),
        OperationStatus::Message(message) => message.clone(),
    };
    let help = match &app.mode {
        Mode::Normal if app.active_view == AnalysisView::Advice => {
            "[tab] view  [g] goal  [r] refresh  [a/e/d] records  [q] quit"
        }
        Mode::Normal => "[tab] view  [r] refresh  [a/e/d] records  [q] quit",
        Mode::Adding(_) | Mode::Editing(_) => "[tab] field  [enter] save  [esc] cancel",
        Mode::ConfirmDelete => "[enter] delete  [esc] cancel",
    };

    frame.render_widget(
        Paragraph::new(vec![Line::from(status), Line::from(help)]).block(panel("Status")),
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
        .title(title)
}

fn view_tabs(active: AnalysisView) -> String {
    [
        AnalysisView::Summary,
        AnalysisView::Compare,
        AnalysisView::Advice,
    ]
    .into_iter()
    .map(|view| {
        if view == active {
            format!("[{}]", view.label())
        } else {
            view.label().to_string()
        }
    })
    .collect::<Vec<_>>()
    .join(" ")
}

fn format_average(period: &PeriodAverage) -> String {
    format_optional_kg(period.average_kg)
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
    use crate::cli::AdviceGoal;
    use crate::models::WeightRecord;
    use crate::stats::{DietGoal, build_diet_advice, compare_weights};
    use crate::use_cases::{AdviceResult, CompareWeightsResult};
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
        assert!(RECORDS_WIDTH_PERCENT < ANALYSIS_WIDTH_PERCENT);
    }

    #[test]
    fn renders_analysis_view_tabs() {
        let mut app = App::new_with_date(date("2026-05-19"));
        app.active_view = AnalysisView::Compare;

        let output = render_to_text(&app, 120, 28);

        assert!(output.contains("Summary [Compare] Advice"));
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
