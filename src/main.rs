mod directx;

fn main() {
    let app = directx::DXApplication::new("Window title", 1280, 960, directx::ask_yn("フルスクリーンで起動しますか？", "Question"));
    loop {
        if app.do_event() {
            break;
        }
    }
}
