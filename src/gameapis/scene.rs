use super::{character::*, input::*, requests::*, resource::*};

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

pub struct GameScene {
    start: u32,
    count: u32,
    chara_1p: Character,
    chara_2p: Character,
}

impl GameScene {
    /// Constructor. Super can't use this.
    fn new() -> Scene {
        let rnd: u32 = rand::prelude::random();
        Scene::Game(Self {
            start: (rnd % 300) + 120,
            count: 0,
            chara_1p: Character::new(CharaID::Udonge),
            chara_2p: Character::new(CharaID::Tei),
        })
    }
    /// Update game scene. Return the next state and requests.
    pub fn update(self, keystates: &KeyStates) -> (Scene, Requests) {
        if self.count == self.start {
            println!("bang! {}  {}", self.count, self.start);
        }
        let chara_1p = self.chara_1p.update(keystates.z);
        let chara_2p = self.chara_2p.update(keystates.l);
        let chara_imgrq_1p = chara_1p.get_imgrqs();
        let chara_imgrq_2p = chara_2p.get_imgrqs();
        let reqs = Requests::new()
            .push_imgrq_xy(ImgResID::StageBamboo, 0.0, 0.0, false)
            .push_request(chara_imgrq_1p)
            .push_request(Request::Reverse(true))
            .push_request(chara_imgrq_2p)
            .push_request(Request::Reverse(false));
        (
            Scene::Game(Self {
                start: self.start,
                count: self.count + 1,
                chara_1p,
                chara_2p,
            }),
            reqs,
        )
    }
}
