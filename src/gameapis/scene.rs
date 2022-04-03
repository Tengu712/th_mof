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
        let (imgkey_1p, uvl_1p, uvt_1p) = chara_1p.get_drawinfo();
        let (imgkey_2p, uvl_2p, uvt_2p) = chara_2p.get_drawinfo();
        let reqs = Requests::new()
            .push_imgrq(ImgResID::StageBamboo, 0.0, 0.0, false)
            .push_imgrq_uv(imgkey_1p, 0.0, 0.0, 512.0, 720.0, uvl_1p, uvt_1p, false)
            .push_imgrq_uv(imgkey_2p, 768.0, 0.0, 512.0, 720.0, uvl_2p, uvt_2p, false);
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
