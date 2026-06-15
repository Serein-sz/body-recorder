use super::Action;
use crate::app::use_cases::{self, AdviceResult, CompareWeightsResult, TargetResult, TdeeResult};
use crate::domain::goals::AdviceGoal;
use crate::domain::models::{UserProfile, WeightRecord};
use crate::domain::validation::{parse_date, validate_weight};
use crate::storage::config::read_config;
use crate::storage::repository::WeightRepository;
use chrono::{Duration, Local, NaiveDate};

const RECENT_LIMIT: u32 = 30;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum OperationStatus {
    Idle,
    Loading,
    Saving(&'static str),
    Error(String),
    Message(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InputField {
    Date,
    Weight,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct InputState {
    pub(crate) date: String,
    pub(crate) weight: String,
    pub(crate) field: InputField,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Mode {
    Normal,
    Adding(InputState),
    Editing(InputState),
    ConfirmDelete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AnalysisView {
    Summary,
    Compare,
    Advice,
    Target,
}

impl AnalysisView {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Summary => "Summary",
            Self::Compare => "Compare",
            Self::Advice => "Advice",
            Self::Target => "Target",
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Summary => Self::Compare,
            Self::Compare => Self::Advice,
            Self::Advice => Self::Target,
            Self::Target => Self::Summary,
        }
    }
}

#[derive(Debug)]
pub(crate) enum LoadState<T> {
    NotLoaded,
    Loading,
    Ready(T),
    Error(String),
}

impl<T> LoadState<T> {
    fn invalidate(&mut self) {
        *self = Self::NotLoaded;
    }
}

#[derive(Debug)]
pub(crate) struct App {
    pub(crate) records: Vec<WeightRecord>,
    pub(crate) selected: usize,
    pub(crate) status: OperationStatus,
    pub(crate) mode: Mode,
    pub(crate) should_quit: bool,
    pub(crate) reference_date: NaiveDate,
    pub(crate) active_view: AnalysisView,
    pub(crate) compare: LoadState<CompareWeightsResult>,
    pub(crate) advice: LoadState<AdviceResult>,
    pub(crate) target: LoadState<TargetResult>,
    pub(crate) tdee: LoadState<TdeeResult>,
    pub(crate) advice_goal: AdviceGoal,
    pub(crate) profile: UserProfile,
}

impl App {
    pub(crate) fn new() -> Self {
        let profile = read_config().map(|c| c.profile).unwrap_or_default();
        let mut app = Self::new_with_date(Local::now().date_naive());
        app.profile = profile;
        app
    }

    pub(crate) fn new_with_date(reference_date: NaiveDate) -> Self {
        Self {
            records: Vec::new(),
            selected: 0,
            status: OperationStatus::Idle,
            mode: Mode::Normal,
            should_quit: false,
            reference_date,
            active_view: AnalysisView::Summary,
            compare: LoadState::NotLoaded,
            advice: LoadState::NotLoaded,
            target: LoadState::NotLoaded,
            tdee: LoadState::NotLoaded,
            advice_goal: AdviceGoal::Cut,
            profile: UserProfile::default(),
        }
    }

    pub(crate) fn start_loading(&mut self) {
        self.status = OperationStatus::Loading;
    }

    pub(crate) async fn load_recent(&mut self, repository: &impl WeightRepository) {
        self.status = OperationStatus::Loading;
        match use_cases::list_weights(repository, RECENT_LIMIT).await {
            Ok(result) => {
                self.records = result.records;
                self.clamp_selection();
                self.load_tdee_quietly(repository).await;
                self.status = OperationStatus::Idle;
            }
            Err(error) => self.status = OperationStatus::Error(error.to_string()),
        }
    }

    pub(crate) async fn handle_action(
        &mut self,
        action: Action,
        repository: &impl WeightRepository,
    ) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::Refresh if matches!(self.mode, Mode::Normal) => {
                self.refresh(repository).await;
            }
            Action::ToggleField if matches!(self.mode, Mode::Normal) => {
                self.active_view = self.active_view.next();
                self.load_active_analysis(repository).await;
            }
            Action::RotateAdviceGoal
                if matches!(self.mode, Mode::Normal)
                    && self.active_view == AnalysisView::Advice =>
            {
                self.advice_goal = next_advice_goal(self.advice_goal);
                self.load_advice(repository).await;
            }
            Action::Add if matches!(self.mode, Mode::Normal) => self.start_add(),
            Action::Edit if matches!(self.mode, Mode::Normal) => self.start_edit(),
            Action::Delete if matches!(self.mode, Mode::Normal) => self.start_delete(),
            Action::Up => self.move_up(),
            Action::Down => self.move_down(),
            Action::Cancel => self.cancel_or_clear(),
            Action::ToggleField => self.toggle_field(),
            Action::Backspace => self.backspace(),
            Action::Input(value) => self.input(value),
            Action::PreviousDate => self.adjust_add_date(-1),
            Action::NextDate => self.adjust_add_date(1),
            Action::Confirm => self.confirm(repository).await,
            _ => {}
        }
    }

    fn start_add(&mut self) {
        self.status = OperationStatus::Idle;
        let weight = self
            .latest_record()
            .map(|record| format!("{:.2}", record.weight_kg))
            .unwrap_or_default();
        self.mode = Mode::Adding(InputState {
            date: String::new(),
            weight,
            field: InputField::Weight,
        });
    }

    fn start_edit(&mut self) {
        let Some(record) = self.selected_record().cloned() else {
            self.status = OperationStatus::Error("no record selected".to_string());
            return;
        };

        self.status = OperationStatus::Idle;
        self.mode = Mode::Editing(InputState {
            date: record.record_date.to_string(),
            weight: format!("{:.2}", record.weight_kg),
            field: InputField::Weight,
        });
    }

    fn start_delete(&mut self) {
        if self.selected_record().is_some() {
            self.status = OperationStatus::Idle;
            self.mode = Mode::ConfirmDelete;
        } else {
            self.status = OperationStatus::Error("no record selected".to_string());
        }
    }

    fn move_up(&mut self) {
        match &mut self.mode {
            Mode::Normal if self.selected > 0 => self.selected -= 1,
            Mode::Adding(input) | Mode::Editing(input) if input.field == InputField::Weight => {
                adjust_weight_input(input, 0.1);
            }
            _ => {}
        }
    }

    fn move_down(&mut self) {
        match &mut self.mode {
            Mode::Normal if self.selected + 1 < self.records.len() => self.selected += 1,
            Mode::Adding(input) | Mode::Editing(input) if input.field == InputField::Weight => {
                adjust_weight_input(input, -0.1);
            }
            _ => {}
        }
    }

    fn cancel_or_clear(&mut self) {
        if matches!(self.status, OperationStatus::Error(_)) {
            self.status = OperationStatus::Idle;
        }
        self.mode = Mode::Normal;
    }

    fn toggle_field(&mut self) {
        match &mut self.mode {
            Mode::Adding(input) => {
                input.field = match input.field {
                    InputField::Date => InputField::Weight,
                    InputField::Weight => InputField::Date,
                };
            }
            Mode::Editing(input) => input.field = InputField::Weight,
            _ => {}
        }
    }

    fn input(&mut self, value: char) {
        let target = match &mut self.mode {
            Mode::Adding(input) | Mode::Editing(input) => match input.field {
                InputField::Date => &mut input.date,
                InputField::Weight => &mut input.weight,
            },
            _ => return,
        };

        if value.is_ascii_digit() || matches!(value, '.' | '-') {
            target.push(value);
        }
    }

    fn backspace(&mut self) {
        let target = match &mut self.mode {
            Mode::Adding(input) | Mode::Editing(input) => match input.field {
                InputField::Date => &mut input.date,
                InputField::Weight => &mut input.weight,
            },
            _ => return,
        };

        target.pop();
    }

    fn adjust_add_date(&mut self, days: i64) {
        let Mode::Adding(input) = &mut self.mode else {
            return;
        };
        if input.field != InputField::Date {
            return;
        }

        let base_date = if input.date.trim().is_empty() {
            self.reference_date
        } else {
            match parse_date(input.date.trim()) {
                Ok(date) => date,
                Err(_) => return,
            }
        };

        input.date = (base_date + Duration::days(days)).to_string();
    }

    async fn confirm(&mut self, repository: &impl WeightRepository) {
        let mode = self.mode.clone();
        match mode {
            Mode::Adding(input) => self.submit_add(input, repository).await,
            Mode::Editing(input) => self.submit_edit(input, repository).await,
            Mode::ConfirmDelete => self.submit_delete(repository).await,
            Mode::Normal => {}
        }
    }

    async fn refresh(&mut self, repository: &impl WeightRepository) {
        match self.active_view {
            AnalysisView::Summary => self.refresh_summary(repository).await,
            AnalysisView::Compare => self.load_compare(repository).await,
            AnalysisView::Advice => self.load_advice(repository).await,
            AnalysisView::Target => self.load_target(repository).await,
        }
    }

    async fn refresh_summary(&mut self, repository: &impl WeightRepository) {
        self.status = OperationStatus::Loading;
        match use_cases::list_weights(repository, RECENT_LIMIT).await {
            Ok(result) => {
                self.records = result.records;
                self.clamp_selection();
                self.load_tdee_quietly(repository).await;
                self.status = OperationStatus::Message("refreshed summary".to_string());
            }
            Err(error) => self.status = OperationStatus::Error(error.to_string()),
        }
    }

    async fn load_active_analysis(&mut self, repository: &impl WeightRepository) {
        match self.active_view {
            AnalysisView::Summary if matches!(self.tdee, LoadState::NotLoaded) => {
                self.load_tdee(repository).await;
            }
            AnalysisView::Compare if matches!(self.compare, LoadState::NotLoaded) => {
                self.load_compare(repository).await;
            }
            AnalysisView::Advice if matches!(self.advice, LoadState::NotLoaded) => {
                self.load_advice(repository).await;
            }
            AnalysisView::Target if matches!(self.target, LoadState::NotLoaded) => {
                self.load_target(repository).await;
            }
            AnalysisView::Summary
            | AnalysisView::Compare
            | AnalysisView::Advice
            | AnalysisView::Target => {}
        }
    }

    pub(crate) async fn load_compare(&mut self, repository: &impl WeightRepository) {
        self.compare = LoadState::Loading;
        match use_cases::compare(repository, Some(self.reference_date.to_string())).await {
            Ok(result) => {
                self.compare = LoadState::Ready(result);
                self.status = OperationStatus::Message("loaded compare analysis".to_string());
            }
            Err(error) => self.compare = LoadState::Error(error.to_string()),
        }
    }

    pub(crate) async fn load_advice(&mut self, repository: &impl WeightRepository) {
        self.advice = LoadState::Loading;
        match use_cases::advice(
            repository,
            Some(self.advice_goal),
            Some(self.reference_date.to_string()),
        )
        .await
        {
            Ok(result) => {
                self.advice = LoadState::Ready(result);
                self.status = OperationStatus::Message("loaded advice analysis".to_string());
            }
            Err(error) => self.advice = LoadState::Error(error.to_string()),
        }
    }

    pub(crate) async fn load_target(&mut self, repository: &impl WeightRepository) {
        self.target = LoadState::Loading;
        match use_cases::target(
            repository,
            self.profile.target_weight_kg,
            Some(self.reference_date.to_string()),
        )
        .await
        {
            Ok(result) => {
                self.target = LoadState::Ready(result);
                self.status = OperationStatus::Message("loaded target estimate".to_string());
            }
            Err(error) => self.target = LoadState::Error(error.to_string()),
        }
    }

    pub(crate) async fn load_tdee(&mut self, repository: &impl WeightRepository) {
        self.tdee = LoadState::Loading;
        match use_cases::tdee(repository, &self.profile, Some(self.reference_date.to_string())).await {
            Ok(result) => {
                self.tdee = LoadState::Ready(result);
                self.status = OperationStatus::Message("loaded TDEE estimate".to_string());
            }
            Err(error) => self.tdee = LoadState::Error(error.to_string()),
        }
    }

    async fn load_tdee_quietly(&mut self, repository: &impl WeightRepository) {
        self.tdee = LoadState::Loading;
        match use_cases::tdee(repository, &self.profile, Some(self.reference_date.to_string())).await {
            Ok(result) => self.tdee = LoadState::Ready(result),
            Err(error) => self.tdee = LoadState::Error(error.to_string()),
        }
    }

    async fn submit_add(&mut self, input: InputState, repository: &impl WeightRepository) {
        let Some(weight) = parse_weight_input(&input.weight) else {
            self.status = OperationStatus::Error("invalid weight".to_string());
            return;
        };

        let date = if input.date.trim().is_empty() {
            None
        } else if let Err(error) = parse_date(input.date.trim()) {
            self.status = OperationStatus::Error(error.to_string());
            return;
        } else {
            Some(input.date.trim().to_string())
        };

        self.status = OperationStatus::Saving("saving record");
        match use_cases::add_weight(repository, weight, date).await {
            Ok(_) => {
                self.mode = Mode::Normal;
                self.invalidate_analysis();
                self.refresh_after_write(repository, "saved record").await;
            }
            Err(error) => self.status = OperationStatus::Error(error.to_string()),
        }
    }

    async fn submit_edit(&mut self, input: InputState, repository: &impl WeightRepository) {
        let Some(weight) = parse_weight_input(&input.weight) else {
            self.status = OperationStatus::Error("invalid weight".to_string());
            return;
        };
        if let Err(error) = parse_date(&input.date) {
            self.status = OperationStatus::Error(error.to_string());
            return;
        }

        self.status = OperationStatus::Saving("updating record");
        match use_cases::update_weight(repository, input.date, weight).await {
            Ok(_) => {
                self.mode = Mode::Normal;
                self.invalidate_analysis();
                self.refresh_after_write(repository, "updated record").await;
            }
            Err(error) => self.status = OperationStatus::Error(error.to_string()),
        }
    }

    async fn submit_delete(&mut self, repository: &impl WeightRepository) {
        let Some(record) = self.selected_record() else {
            self.status = OperationStatus::Error("no record selected".to_string());
            return;
        };
        let date = record.record_date.to_string();

        self.status = OperationStatus::Saving("deleting record");
        match use_cases::delete_weight(repository, date).await {
            Ok(_) => {
                self.mode = Mode::Normal;
                self.invalidate_analysis();
                self.refresh_after_write(repository, "deleted record").await;
            }
            Err(error) => self.status = OperationStatus::Error(error.to_string()),
        }
    }

    async fn refresh_after_write(&mut self, repository: &impl WeightRepository, message: &str) {
        match use_cases::list_weights(repository, RECENT_LIMIT).await {
            Ok(result) => {
                self.records = result.records;
                self.clamp_selection();
                self.status = OperationStatus::Message(message.to_string());
            }
            Err(error) => self.status = OperationStatus::Error(error.to_string()),
        }
    }

    fn selected_record(&self) -> Option<&WeightRecord> {
        self.records.get(self.selected)
    }

    fn latest_record(&self) -> Option<&WeightRecord> {
        self.records.iter().max_by_key(|record| record.record_date)
    }

    fn invalidate_analysis(&mut self) {
        self.compare.invalidate();
        self.advice.invalidate();
        self.target.invalidate();
        self.tdee.invalidate();
    }

    fn clamp_selection(&mut self) {
        if self.records.is_empty() {
            self.selected = 0;
        } else if self.selected >= self.records.len() {
            self.selected = self.records.len() - 1;
        }
    }
}

pub(crate) fn advice_goal_label(goal: AdviceGoal) -> &'static str {
    match goal {
        AdviceGoal::Cut => "fat loss",
        AdviceGoal::Maintain => "maintenance",
        AdviceGoal::Gain => "weight gain",
    }
}

fn next_advice_goal(goal: AdviceGoal) -> AdviceGoal {
    match goal {
        AdviceGoal::Cut => AdviceGoal::Maintain,
        AdviceGoal::Maintain => AdviceGoal::Gain,
        AdviceGoal::Gain => AdviceGoal::Cut,
    }
}

fn parse_weight_input(value: &str) -> Option<f64> {
    let parsed = value.trim().parse::<f64>().ok()?;
    validate_weight(parsed).ok()
}

fn adjust_weight_input(input: &mut InputState, delta_kg: f64) {
    let Some(weight) = parse_weight_input(&input.weight) else {
        return;
    };

    let adjusted = weight + delta_kg;
    if validate_weight(adjusted).is_ok() {
        input.weight = format!("{adjusted:.2}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppResult;
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
            let record = record(record_date, weight_kg);
            self.records.lock().unwrap().insert(0, record.clone());
            Ok(record)
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
            let mut records = self.records.lock().unwrap();
            for record in records.iter_mut() {
                if record.record_date == record_date {
                    record.weight_kg = weight_kg;
                    return Ok(vec![record.clone()]);
                }
            }
            Ok(Vec::new())
        }

        async fn delete_weight(&self, record_date: NaiveDate) -> AppResult<Vec<WeightRecord>> {
            self.calls
                .lock()
                .unwrap()
                .push(format!("delete:{record_date}"));
            let mut records = self.records.lock().unwrap();
            let Some(index) = records
                .iter()
                .position(|record| record.record_date == record_date)
            else {
                return Ok(Vec::new());
            };
            Ok(vec![records.remove(index)])
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

    #[test]
    fn state_navigation_changes_selection_without_storage() {
        let repository = FakeRepository::new(Vec::new());
        let mut app = App::new_with_date(date("2026-05-19"));
        app.records = vec![
            record(date("2026-05-19"), 72.0),
            record(date("2026-05-18"), 72.5),
        ];

        app.move_down();
        app.move_down();
        app.move_up();

        assert_eq!(app.selected, 0);
        assert!(repository.calls().is_empty());
    }

    #[tokio::test]
    async fn invalid_add_weight_does_not_call_storage() {
        let repository = FakeRepository::new(Vec::new());
        let mut app = App::new_with_date(date("2026-05-19"));

        app.handle_action(Action::Add, &repository).await;
        app.handle_action(Action::Input('0'), &repository).await;
        app.handle_action(Action::Confirm, &repository).await;

        assert!(matches!(app.status, OperationStatus::Error(_)));
        assert!(repository.calls().is_empty());
    }

    #[tokio::test]
    async fn invalid_edit_weight_does_not_call_storage() {
        let repository = FakeRepository::new(vec![record(date("2026-05-19"), 72.0)]);
        let mut app = App::new_with_date(date("2026-05-19"));
        app.records = vec![record(date("2026-05-19"), 72.0)];

        app.handle_action(Action::Edit, &repository).await;
        app.handle_action(Action::Backspace, &repository).await;
        app.handle_action(Action::Backspace, &repository).await;
        app.handle_action(Action::Backspace, &repository).await;
        app.handle_action(Action::Backspace, &repository).await;
        app.handle_action(Action::Backspace, &repository).await;
        app.handle_action(Action::Confirm, &repository).await;

        assert!(matches!(app.status, OperationStatus::Error(_)));
        assert!(repository.calls().is_empty());
    }

    #[tokio::test]
    async fn add_form_prefills_latest_weight_by_record_date() {
        let repository = FakeRepository::new(Vec::new());
        let mut app = App::new_with_date(date("2026-05-19"));
        app.records = vec![
            record(date("2026-05-18"), 72.2),
            record(date("2026-05-20"), 71.9),
            record(date("2026-05-19"), 72.0),
        ];

        app.handle_action(Action::Add, &repository).await;

        match &app.mode {
            Mode::Adding(input) => assert_eq!(input.weight, "71.90"),
            _ => panic!("expected adding mode"),
        }
        assert!(repository.calls().is_empty());
    }

    #[tokio::test]
    async fn weight_field_arrows_adjust_by_tenth_kg() {
        let repository = FakeRepository::new(Vec::new());
        let mut app = App::new_with_date(date("2026-05-19"));
        app.mode = Mode::Adding(InputState {
            date: String::new(),
            weight: "72.00".to_string(),
            field: InputField::Weight,
        });

        app.handle_action(Action::Up, &repository).await;
        app.handle_action(Action::Up, &repository).await;
        app.handle_action(Action::Down, &repository).await;

        match &app.mode {
            Mode::Adding(input) => assert_eq!(input.weight, "72.10"),
            _ => panic!("expected adding mode"),
        }
        assert!(repository.calls().is_empty());
    }

    #[tokio::test]
    async fn weight_arrows_ignore_invalid_or_unfocused_weight_input() {
        let repository = FakeRepository::new(Vec::new());
        let mut app = App::new_with_date(date("2026-05-19"));
        app.mode = Mode::Adding(InputState {
            date: String::new(),
            weight: "invalid".to_string(),
            field: InputField::Weight,
        });

        app.handle_action(Action::Up, &repository).await;
        match &app.mode {
            Mode::Adding(input) => assert_eq!(input.weight, "invalid"),
            _ => panic!("expected adding mode"),
        }

        app.mode = Mode::Adding(InputState {
            date: String::new(),
            weight: "72.00".to_string(),
            field: InputField::Date,
        });
        app.handle_action(Action::Up, &repository).await;
        match &app.mode {
            Mode::Adding(input) => assert_eq!(input.weight, "72.00"),
            _ => panic!("expected adding mode"),
        }
        assert!(repository.calls().is_empty());
    }

    #[tokio::test]
    async fn delete_selected_record_uses_existing_use_case() {
        let repository = FakeRepository::new(vec![record(date("2026-05-19"), 72.0)]);
        let mut app = App::new_with_date(date("2026-05-19"));
        app.records = vec![record(date("2026-05-19"), 72.0)];

        app.handle_action(Action::Delete, &repository).await;
        app.handle_action(Action::Confirm, &repository).await;

        assert_eq!(repository.calls(), ["delete:2026-05-19", "list:30"]);
        assert!(app.records.is_empty());
    }

    #[tokio::test]
    async fn switches_analysis_views_and_loads_compare_advice_target_then_summary() {
        let repository = FakeRepository::new(vec![record(date("2026-05-19"), 72.0)]);
        let mut app = App::new_with_date(date("2026-05-19"));

        app.handle_action(Action::ToggleField, &repository).await;
        assert_eq!(app.active_view, AnalysisView::Compare);
        assert!(matches!(app.compare, LoadState::Ready(_)));

        app.handle_action(Action::ToggleField, &repository).await;
        assert_eq!(app.active_view, AnalysisView::Advice);
        assert!(matches!(app.advice, LoadState::Ready(_)));

        app.handle_action(Action::ToggleField, &repository).await;
        assert_eq!(app.active_view, AnalysisView::Target);
        assert!(matches!(app.target, LoadState::Ready(_)));

        app.handle_action(Action::ToggleField, &repository).await;
        assert_eq!(app.active_view, AnalysisView::Summary);
        assert!(matches!(app.tdee, LoadState::Ready(_)));
        assert_eq!(
            repository.calls(),
            [
                "between:2025-05-19:2026-05-19",
                "between:2026-04-22:2026-05-19",
                "between:2026-04-22:2026-05-19",
                "between:2026-05-13:2026-05-19"
            ]
        );
    }

    #[tokio::test]
    async fn analysis_view_controls_are_ignored_during_input_modes() {
        let repository = FakeRepository::new(Vec::new());
        let mut app = App::new_with_date(date("2026-05-19"));

        app.handle_action(Action::Add, &repository).await;
        app.handle_action(Action::ToggleField, &repository).await;
        app.handle_action(Action::RotateAdviceGoal, &repository)
            .await;

        assert_eq!(app.active_view, AnalysisView::Summary);
        assert_eq!(app.advice_goal, AdviceGoal::Cut);
        match app.mode {
            Mode::Adding(input) => assert_eq!(input.field, InputField::Date),
            _ => panic!("expected adding mode"),
        }
        assert!(repository.calls().is_empty());
    }

    #[tokio::test]
    async fn add_date_arrows_use_reference_date_when_blank() {
        let repository = FakeRepository::new(Vec::new());
        let mut app = App::new_with_date(date("2026-05-19"));

        app.handle_action(Action::Add, &repository).await;
        app.handle_action(Action::ToggleField, &repository).await;
        app.handle_action(Action::PreviousDate, &repository).await;

        match &app.mode {
            Mode::Adding(input) => assert_eq!(input.date, "2026-05-18"),
            _ => panic!("expected adding mode"),
        }

        app.handle_action(Action::NextDate, &repository).await;
        app.handle_action(Action::NextDate, &repository).await;

        match &app.mode {
            Mode::Adding(input) => assert_eq!(input.date, "2026-05-20"),
            _ => panic!("expected adding mode"),
        }
        assert!(repository.calls().is_empty());
    }

    #[tokio::test]
    async fn add_date_arrows_adjust_valid_existing_date() {
        let repository = FakeRepository::new(Vec::new());
        let mut app = App::new_with_date(date("2026-05-19"));
        app.mode = Mode::Adding(InputState {
            date: "2026-06-01".to_string(),
            weight: String::new(),
            field: InputField::Date,
        });

        app.handle_action(Action::PreviousDate, &repository).await;
        app.handle_action(Action::PreviousDate, &repository).await;
        app.handle_action(Action::NextDate, &repository).await;

        match &app.mode {
            Mode::Adding(input) => assert_eq!(input.date, "2026-05-31"),
            _ => panic!("expected adding mode"),
        }
        assert!(repository.calls().is_empty());
    }

    #[tokio::test]
    async fn add_date_arrows_leave_invalid_date_unchanged() {
        let repository = FakeRepository::new(Vec::new());
        let mut app = App::new_with_date(date("2026-05-19"));
        app.mode = Mode::Adding(InputState {
            date: "2026-99-99".to_string(),
            weight: String::new(),
            field: InputField::Date,
        });

        app.handle_action(Action::NextDate, &repository).await;

        match &app.mode {
            Mode::Adding(input) => assert_eq!(input.date, "2026-99-99"),
            _ => panic!("expected adding mode"),
        }
        assert!(repository.calls().is_empty());
    }

    #[tokio::test]
    async fn date_arrows_are_scoped_to_add_date_field() {
        let repository = FakeRepository::new(vec![record(date("2026-05-19"), 72.0)]);
        let mut app = App::new_with_date(date("2026-05-19"));

        app.handle_action(Action::NextDate, &repository).await;
        assert!(matches!(app.mode, Mode::Normal));

        app.handle_action(Action::Add, &repository).await;
        app.handle_action(Action::NextDate, &repository).await;
        match &app.mode {
            Mode::Adding(input) => {
                assert_eq!(input.field, InputField::Weight);
                assert_eq!(input.date, "");
            }
            _ => panic!("expected adding mode"),
        }

        app.records = vec![record(date("2026-05-19"), 72.0)];
        app.mode = Mode::Normal;
        app.handle_action(Action::Edit, &repository).await;
        app.handle_action(Action::ToggleField, &repository).await;
        app.handle_action(Action::PreviousDate, &repository).await;
        match &app.mode {
            Mode::Editing(input) => assert_eq!(input.date, "2026-05-19"),
            _ => panic!("expected editing mode"),
        }
        assert!(repository.calls().is_empty());
    }

    #[tokio::test]
    async fn advice_goal_rotates_and_reloads_only_in_advice_view() {
        let repository = FakeRepository::new(vec![record(date("2026-05-19"), 72.0)]);
        let mut app = App::new_with_date(date("2026-05-19"));

        app.handle_action(Action::RotateAdviceGoal, &repository)
            .await;
        assert_eq!(app.advice_goal, AdviceGoal::Cut);
        assert!(repository.calls().is_empty());

        app.active_view = AnalysisView::Advice;
        app.handle_action(Action::RotateAdviceGoal, &repository)
            .await;

        assert_eq!(app.advice_goal, AdviceGoal::Maintain);
        assert!(matches!(app.advice, LoadState::Ready(_)));
        assert_eq!(repository.calls(), ["between:2026-04-22:2026-05-19"]);
    }

    #[tokio::test]
    async fn refresh_loads_tdee_when_summary_view_is_active() {
        let repository = FakeRepository::new(vec![record(date("2026-05-19"), 72.0)]);
        let mut app = App::new_with_date(date("2026-05-19"));

        app.handle_action(Action::Refresh, &repository).await;

        assert!(matches!(app.tdee, LoadState::Ready(_)));
        assert_eq!(
            repository.calls(),
            ["list:30", "between:2026-05-13:2026-05-19"]
        );
    }

    #[tokio::test]
    async fn refresh_uses_active_analysis_view() {
        let repository = FakeRepository::new(vec![record(date("2026-05-19"), 72.0)]);
        let mut app = App::new_with_date(date("2026-05-19"));

        app.handle_action(Action::Refresh, &repository).await;
        app.active_view = AnalysisView::Compare;
        app.handle_action(Action::Refresh, &repository).await;
        app.active_view = AnalysisView::Advice;
        app.handle_action(Action::Refresh, &repository).await;
        app.active_view = AnalysisView::Target;
        app.handle_action(Action::Refresh, &repository).await;

        assert_eq!(
            repository.calls(),
            [
                "list:30",
                "between:2026-05-13:2026-05-19",
                "between:2025-05-19:2026-05-19",
                "between:2026-04-22:2026-05-19",
                "between:2026-04-22:2026-05-19"
            ]
        );
    }

    #[tokio::test]
    async fn record_write_invalidates_loaded_analysis() {
        let repository = FakeRepository::new(vec![record(date("2026-05-19"), 72.0)]);
        let mut app = App::new_with_date(date("2026-05-19"));
        app.records = vec![record(date("2026-05-19"), 72.0)];
        app.load_compare(&repository).await;
        app.load_advice(&repository).await;
        app.load_target(&repository).await;
        app.load_tdee(&repository).await;

        app.handle_action(Action::Edit, &repository).await;
        app.handle_action(Action::Input('1'), &repository).await;
        app.handle_action(Action::Confirm, &repository).await;

        assert!(matches!(app.compare, LoadState::NotLoaded));
        assert!(matches!(app.advice, LoadState::NotLoaded));
        assert!(matches!(app.target, LoadState::NotLoaded));
        assert!(matches!(app.tdee, LoadState::NotLoaded));
    }
}
