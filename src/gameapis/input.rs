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
        let mut self_mut = self;
        match code {
            KeyCode::Z => self_mut.z = get_next_keystate(0x5A, self_mut.z),
            KeyCode::L => self_mut.l = get_next_keystate(0x4C, self_mut.l),
        };
        self_mut
    }
}
