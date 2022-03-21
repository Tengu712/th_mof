use super::{
    super::winapis::input::*,
    {requests::*, resources::*},
};

/// A eunm for scene that's matched at mainloop.
pub enum Scene {
    Title(TitleScene),
    Game(GameScene),
}

pub struct TitleScene {}

impl TitleScene {
    /// Return TitleScene wraped in Scene.
    /// Super module can call this and start running scenes.
    pub fn new() -> Scene {
        Scene::Title(Self {})
    }
    /// Update title scene. Return the next state and requests.
    pub fn update(self, keystate: &KeyStates) -> (Scene, Requests) {
        (GameScene::new(), Requests::new())
    }
}

pub struct GameScene {
    start: u32,
    count: u32,
}

impl GameScene {
    /// Constructor. Super can't use this.
    fn new() -> Scene {
        let rnd: u32 = rand::prelude::random();
        Scene::Game(Self {
            start: (rnd % 300) + 120,
            count: 0,
        })
    }
    /// Update game scene. Return the next state and requests.
    pub fn update(self, keystates: &KeyStates) -> (Scene, Requests) {
        if self.count == self.start {
            println!("bang! {}  {}", self.count, self.start);
        }
        if keystates.space > 0 {
            println!("space");
        }
        (
            Scene::Game(Self {
                start: self.start,
                count: self.count + 1,
            }),
            Requests::new(),
        )
    }
}
