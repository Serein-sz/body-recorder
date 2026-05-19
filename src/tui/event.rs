use super::Action;
use crate::error::AppResult;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::time::Duration;

pub(crate) fn read_action(timeout: Duration) -> AppResult<Option<Action>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }

    match event::read()? {
        Event::Key(key) => Ok(key_event_to_action(key)),
        _ => Ok(None),
    }
}

pub(crate) fn key_event_to_action(key: KeyEvent) -> Option<Action> {
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('r') => Some(Action::Refresh),
        KeyCode::Char('a') => Some(Action::Add),
        KeyCode::Char('e') => Some(Action::Edit),
        KeyCode::Char('d') => Some(Action::Delete),
        KeyCode::Char('g') => Some(Action::RotateAdviceGoal),
        KeyCode::Char('j') => Some(Action::Down),
        KeyCode::Char('k') => Some(Action::Up),
        KeyCode::Char(value) => Some(Action::Input(value)),
        KeyCode::Backspace => Some(Action::Backspace),
        KeyCode::Down => Some(Action::Down),
        KeyCode::Up => Some(Action::Up),
        KeyCode::Enter => Some(Action::Confirm),
        KeyCode::Esc => Some(Action::Cancel),
        KeyCode::Tab => Some(Action::ToggleField),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEventState, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn maps_command_keys_to_actions() {
        assert_eq!(
            key_event_to_action(key(KeyCode::Char('a'))),
            Some(Action::Add)
        );
        assert_eq!(
            key_event_to_action(key(KeyCode::Char('e'))),
            Some(Action::Edit)
        );
        assert_eq!(
            key_event_to_action(key(KeyCode::Char('d'))),
            Some(Action::Delete)
        );
        assert_eq!(
            key_event_to_action(key(KeyCode::Char('r'))),
            Some(Action::Refresh)
        );
        assert_eq!(
            key_event_to_action(key(KeyCode::Char('q'))),
            Some(Action::Quit)
        );
        assert_eq!(
            key_event_to_action(key(KeyCode::Tab)),
            Some(Action::ToggleField)
        );
        assert_eq!(
            key_event_to_action(key(KeyCode::Char(']'))),
            Some(Action::Input(']'))
        );
        assert_eq!(
            key_event_to_action(key(KeyCode::Char('g'))),
            Some(Action::RotateAdviceGoal)
        );
    }

    #[test]
    fn ignores_non_press_key_events() {
        let mut event = key(KeyCode::Char('a'));
        event.kind = KeyEventKind::Release;

        assert_eq!(key_event_to_action(event), None);
    }
}
