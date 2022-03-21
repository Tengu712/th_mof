use super::super::winapis::winapi::get_next_keystate;

/// A enum for abstract key code.
pub enum KeyCode {
    Space,
}

/// A struct for keeping key states.
#[derive(Clone, Default)]
pub struct KeyStates {
    pub space: i16,
}

impl KeyStates {
    /// Constructor. Initialize all field with 0.
    pub fn new() -> Self {
        Default::default()
    }
    /// Detect key state and return self state.
    pub fn detect(self, code: KeyCode) -> Self {
        let mut cloned = self.clone();
        match code {
            KeyCode::Space => cloned.space = get_next_keystate(32, cloned.space),
        };
        cloned
    }
}
