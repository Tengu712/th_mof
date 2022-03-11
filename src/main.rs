mod directx;

fn main() {
    let app = directx::DXApplication::new("Window title", 1280, 960, true);
    loop {
        if app.do_event() {
            break;
        }
    }
}
