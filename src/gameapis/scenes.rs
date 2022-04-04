pub mod game_ending;
pub mod game_shooting;
pub mod game_talking;
pub mod game_waiting;
pub mod game;

use super::{character::*, input::*, requests::*, resource::*};
use game::*;

/// A eunm for scene that's matched at mainloop.
pub enum Scene {
    Title(TitleScene),
    Game(GameScene),
}

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
