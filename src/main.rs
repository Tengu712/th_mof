pub mod gameapis;
pub mod winapis;

use gameapis::{input::*, requests::*, resource::*, scenes::*};
use std::collections::HashMap;
use winapis::{direct2d::*, winapi::*};

/// A struct for application and resource bank to run the game.
struct Application {
    winapp: WindowsApplication,
    d2dapp: D2DApplication,
    images: HashMap<ImgResID, Image>,
}

impl Application {
    /// Constructor.
    fn new() -> Result<Self, String> {
        let winapp = WindowsApplication::new("Window title", 1280, 720, true)?;
        let d2dapp = D2DApplication::new(&winapp)?;
        let mut images = HashMap::new();
        // Character
        images.insert(
            ImgResID::Udonge,
            d2dapp.create_image_from_file(
                "C:/Users/kazuki/OneDrive/touhou/illust/2022spring/game/udonge.png",
            )?,
        );
        images.insert(
            ImgResID::Tei,
            d2dapp.create_image_from_file(
                "C:/Users/kazuki/OneDrive/touhou/illust/2022spring/game/tei.png",
            )?,
        );
        // Backgrounds
        images.insert(
            ImgResID::StageBamboo,
            d2dapp.create_image_from_file(
                "C:/Users/kazuki/OneDrive/touhou/illust/2022spring/game/stage_bamboo.png",
            )?,
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
        let mut keystates = KeyStates::new();
        let mut scene = title::TitleScene::new();
        while !self.winapp.do_event() {
            keystates = keystates.detect(KeyCode::Z).detect(KeyCode::L);
            let (next, reqs) = match scene {
                Scene::Title(n) => n.update(&keystates),
                Scene::Game(n) => n.update(&keystates),
            };
            scene = next;
            self.d2dapp.begin_draw();
            self.d2dapp.clear_screen(0.0, 0.0, 0.0);
            for req in reqs.get_array().iter() {
                self.do_request(req)?;
            }
            self.d2dapp.end_draw()?;
            self.d2dapp.present(1, 0)?;
        }
        Ok(())
    }
    /// **[Side Effect]**
    /// Do requests of drawing image or.
    fn do_request(&self, request: &Request) -> Result<(), String> {
        match request {
            Request::Reverse(n) => self.d2dapp.reverse(n.clone(), 1280.0),
            Request::Image(n) => {
                let image = self
                    .images
                    .get(&n.key)
                    .ok_or(format!("{} : {:?}", "Invalid draw request.", &n.key))?;
                let width = n.width.unwrap_or(image.width as f32);
                let height = n.height.unwrap_or(image.height as f32);
                let uv_width = n.uv_width.unwrap_or(image.width as f32);
                let uv_height = n.uv_height.unwrap_or(image.height as f32);
                self.d2dapp.draw_image(
                    image, n.left, n.top, width, height, n.uv_left, n.uv_top, uv_width, uv_height,
                    n.alpha, n.center,
                );
            }
            _ => (),
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
