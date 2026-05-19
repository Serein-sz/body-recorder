use super::{App, InputField, Mode, OperationStatus};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};

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
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(root[1]);

    render_records(frame, app, body[0]);
    render_trend(frame, app, body[1]);
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
        "Body Recorder   records: {}   selected: {}   mode: {}",
        app.records.len(),
        if app.records.is_empty() {
            "-".to_string()
        } else {
            (app.selected + 1).to_string()
        },
        mode
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

fn render_trend(frame: &mut Frame<'_>, app: &App, area: ratatui::layout::Rect) {
    let lines = app
        .trend_lines()
        .into_iter()
        .map(Line::from)
        .collect::<Vec<_>>();
    frame.render_widget(
        Paragraph::new(lines)
            .block(panel("Trend"))
            .wrap(Wrap { trim: false }),
        area,
    );
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
        Mode::Normal => "[a] add  [e] edit  [d] delete  [r] refresh  [q] quit",
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
    use crate::models::WeightRecord;
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
}
