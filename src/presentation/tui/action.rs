#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Action {
    Add,
    Backspace,
    Cancel,
    Confirm,
    Delete,
    Down,
    Edit,
    Input(char),
    Quit,
    Refresh,
    RotateAdviceGoal,
    ToggleField,
    Up,
}
