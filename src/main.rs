pub mod winapis;

use winapis::{direct2d::*, winapi::*};

fn main() {
    let winapp = WindowsApplication::new(
        "Window title",
        1280,
        960,
        ask_yesno(
            "フルスクリーンで起動しますか？".to_owned(),
            "Question".to_owned(),
        ),
    )
    .map_err(|e| show_messagebox(e.to_string(), "Error".to_owned()))
    .unwrap();
    let d2dapp = D2DApplication::new(&winapp)
        .map_err(|e| show_messagebox(e.to_string(), "Error".to_owned()))
        .unwrap();
    d2dapp.clear_screen();
    loop {
        if winapp.do_event() {
            break;
        }
    }
}
