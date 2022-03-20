pub mod gameapis;
pub mod winapis;

use gameapis::{resources::*, scenes::*};
use std::collections::HashMap;
use winapis::{direct2d::*, winapi::*};

/// A struct for application and resource bank to run the game.
struct Application {
    winapp: WindowsApplication,
    d2dapp: D2DApplication,
    images: HashMap<String, Image>,
}

impl Application {
    /// Constructor.
    fn new() -> Result<Self, String> {
        let winapp = WindowsApplication::new(
            "Window title",
            1280,
            720,
            ask_yesno("フルスクリーンで起動しますか？", "Question"),
        )?;
        let d2dapp = D2DApplication::new(&winapp)?;
        let mut images = HashMap::new();
        images.insert(
            String::from("a"),
            d2dapp.create_image_from_file("img/a.png")?,
        );
        Ok(Self {
            winapp,
            d2dapp,
            images,
        })
    }

    /// **[Side Effect]**
    /// Run the game.
    fn run(self) -> Result<(), String> {
        let mut scene = Scene::new();
        loop {
            if self.winapp.do_event() {
                break;
            }
            let (next, reqs) = scene.update();
            scene = next;
            self.d2dapp.begin_draw();
            self.d2dapp.clear_screen(0.0, 0.0, 0.0);
            for req in reqs {
                self.do_request(req)?;
            }
            self.d2dapp.end_draw()?;
            self.d2dapp.present(1, 0)?;
        }
        Ok(())
    }

    /// **[Side Effect]**
    /// Do requests of drawing image or.
    fn do_request(&self, req: Request) -> Result<(), String> {
        match req {
            Request::Image(n) => {
                let image = self
                    .images
                    .get(n.key.as_str())
                    .ok_or(n.key + "\nInvalid draw request.")?;
                let width = n.width.unwrap_or(image.width as f32);
                let height = n.height.unwrap_or(image.height as f32);
                let uv_width = n.uv_width.unwrap_or(image.width as f32);
                let uv_height = n.uv_height.unwrap_or(image.height as f32);
                self.d2dapp.draw_image(
                    image, n.left, n.top, width, height, n.uv_left, n.uv_top, uv_width, uv_height,
                    n.center,
                );
            }
        }
        Ok(())
    }
}

/// Another entry point that's to return error to main function.
fn main_with_result() -> Result<(), String> {
    Application::new()?.run()?;
    Ok(())
}

/// Entry point. Unwrap application error here.
fn main() {
    match main_with_result() {
        Ok(()) => (),
        Err(e) => show_messagebox(e.as_str(), "Error"),
    }
}
