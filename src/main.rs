pub mod winapis;

use winapis::{direct2d::*, winapi::*};

fn main() {
    let winapp = WindowsApplication::new(
        "Window title",
        1280,
        960,
        ask_yesno("フルスクリーンで起動しますか？", "Question"),
    )
    .unwrap();
    let d2dapp = D2DApplication::new(&winapp).unwrap();
    d2dapp.clear_screen();
    loop {
        if winapp.do_event() {
            break;
        }
    }
}
