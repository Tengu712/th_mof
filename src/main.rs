pub mod winapis;

use winapis::{direct2d::*, winapi::*};

fn main() {
    let winapp = WindowsApplication::new(
        "Window title",
        1280,
        720,
        ask_yesno(
            "フルスクリーンで起動しますか？".to_owned(),
            "Question".to_owned(),
        ),
    )
    .map_err(|e| show_messagebox(e, "Error".to_owned()))
    .unwrap();
    let d2dapp = D2DApplication::new(&winapp)
        .map_err(|e| show_messagebox(e, "Error".to_owned()))
        .unwrap();
    loop {
        if winapp.do_event() {
            break;
        }
        d2dapp.begin_draw();
        d2dapp.clear_screen(0.0, 0.0, 0.0);
        d2dapp
            .end_draw()
            .map_err(|e| show_messagebox(e, "Error".to_owned()))
            .unwrap();
        d2dapp
            .present(1, 0)
            .map_err(|e| show_messagebox(e, "Error".to_owned()))
            .unwrap();
    }
}
