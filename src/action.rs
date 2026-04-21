use engine::{GameAction, InputState, Key};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiemAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

impl GameAction for RequiemAction {
    fn count() -> usize {
        4
    }

    fn index(&self) -> usize {
        *self as usize
    }

    fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Self::MoveLeft),
            1 => Some(Self::MoveRight),
            2 => Some(Self::MoveUp),
            3 => Some(Self::MoveDown),
            _ => None,
        }
    }

    fn move_negative_x() -> Option<Self> {
        Some(Self::MoveLeft)
    }
    fn move_positive_x() -> Option<Self> {
        Some(Self::MoveRight)
    }
    fn move_negative_y() -> Option<Self> {
        Some(Self::MoveUp)
    }
    fn move_positive_y() -> Option<Self> {
        Some(Self::MoveDown)
    }
}

pub fn setup_bindings(input: &mut InputState<RequiemAction>) {
    let map = input.input_map_mut();

    map.bind_key(Key::A, RequiemAction::MoveLeft);
    map.bind_key(Key::Left, RequiemAction::MoveLeft);
    map.bind_key(Key::D, RequiemAction::MoveRight);
    map.bind_key(Key::Right, RequiemAction::MoveRight);
    map.bind_key(Key::W, RequiemAction::MoveUp);
    map.bind_key(Key::Up, RequiemAction::MoveUp);
    map.bind_key(Key::S, RequiemAction::MoveDown);
    map.bind_key(Key::Down, RequiemAction::MoveDown);

    #[cfg(not(target_arch = "wasm32"))]
    {
        use engine::gilrs::Button;
        map.bind_button(Button::DPadUp, RequiemAction::MoveUp);
        map.bind_button(Button::DPadDown, RequiemAction::MoveDown);
        map.bind_button(Button::DPadLeft, RequiemAction::MoveLeft);
        map.bind_button(Button::DPadRight, RequiemAction::MoveRight);
    }
}
