mod action;
mod app;
mod event;
mod terminal;
mod ui;

use crate::error::AppResult;
use crate::repository::WeightRepository;
use std::time::Duration;

pub(crate) use action::Action;
pub(crate) use app::{App, InputField, Mode, OperationStatus};

pub async fn run(repository: &impl WeightRepository) -> AppResult<()> {
    let mut terminal = terminal::TerminalGuard::enter()?;
    let mut app = App::new();

    app.start_loading();
    terminal.draw(|frame| ui::render(frame, &app))?;
    app.load_recent(repository).await;

    while !app.should_quit {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if let Some(action) = event::read_action(Duration::from_millis(200))? {
            app.handle_action(action, repository).await;
        }
    }

    Ok(())
}
