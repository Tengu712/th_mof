use super::{requests::*, resources::*};

/// A eunm for scene that's matched at mainloop.
pub enum Scene {
    Title(TitleScene),
}

pub struct TitleScene {}

impl TitleScene {
    /// Return TitleScene wraped in Scene.
    /// Super module can call this and start running scenes.
    pub fn new() -> Scene {
        Scene::Title(Self {})
    }
    /// Update title scene.
    pub fn update(self) -> (Scene, Requests) {
        let reqs = Requests::new().push_imgrq_wh(ImgResID::A, 0.0, 0.0, 1280.0, 720.0, false);
        (Scene::Title(TitleScene {}), reqs)
    }
}
