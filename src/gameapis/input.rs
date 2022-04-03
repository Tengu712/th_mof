use super::super::winapis::winapi::get_next_keystate;

/// A enum for abstract key code.
pub enum KeyCode {
    Z,
    L,
}

/// A struct for keeping key states.
#[derive(Clone, Default)]
pub struct KeyStates {
    pub z: i16,
    pub l: i16,
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
            KeyCode::Z => cloned.z = get_next_keystate(0x5A, cloned.z),
            KeyCode::L => cloned.l = get_next_keystate(0x4C, cloned.l),
        };
        cloned
    }
}
