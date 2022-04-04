pub mod character;
pub mod input;
pub mod requests;
pub mod resource;
pub mod scenes;

/// A indicator function for make code clear. u32.
fn indicator_u32(arg: u32) -> u32 {
    if arg > 0 {
        1
    } else {
        0
    }
}

/// A indicator function for make code clear. bool.
fn indicator_bool(arg: bool) -> u32 {
    if arg {
        1
    } else {
        0
    }
}
