use super::{
    super::{input::*, requests::*},
    *,
};

pub struct TitleScene {}

impl TitleScene {
    /// Return TitleScene wraped in Scene.
    /// Super module can call this and start running scene.
    pub fn new() -> Scene {
        Scene::Title(Self {})
    }
    /// Update title scene. Return the next state and requests.
    pub fn update(self, keystate: &KeyStates) -> (Scene, Requests) {
        if keystate.z == 1 || keystate.l == 1 {
            return (GameScene::new(), Requests::new());
        }
        (Scene::Title(self), Requests::new())
    }
}
