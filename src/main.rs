pub mod gameapis;
pub mod winapis;

use gameapis::scenes::*;
use winapis::{direct2d::*, winapi::*};

/// Entry point. Unwrap application error here.
fn main() {
    run()
        .map_err(|e| show_messagebox(e, "Error".to_owned()))
        .unwrap();
}

/// Run application. Defined for returning Result to main function.
fn run() -> Result<(), String> {
    let winapp = WindowsApplication::new(
        "Window title",
        1280,
        720,
        ask_yesno(
            "フルスクリーンで起動しますか？".to_owned(),
            "Question".to_owned(),
        ),
    )?;
    let d2dapp = D2DApplication::new(&winapp)?;
    let mut scene = Scene::new()?;
    loop {
        if winapp.do_event() {
            break;
        }
        scene = scene.update()?;
        d2dapp.begin_draw();
        d2dapp.clear_screen(0.0, 0.0, 0.0);
        d2dapp.end_draw()?;
        d2dapp.present(1, 0)?;
    }
    Ok(())
}
