use windows::Win32::UI::Input::KeyboardAndMouse::*;

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
            KeyCode::Space => cloned.space = Self::next(32, cloned.space),
        };
        cloned
    }
    /// Private function.
    /// Get key state and return next value based on previous.
    fn next(vkey: i32, state: i16) -> i16 {
        if (unsafe { GetAsyncKeyState(vkey) } as u16 & 0x8000) > 0 {
            std::cmp::max(state + 1, 1)
        } else if state > 0 {
            -1
        } else {
            0
        }
    }
}
