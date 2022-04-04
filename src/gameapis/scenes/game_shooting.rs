use super::{game::*, *};

impl GameScene {
    ///
    pub fn update_shooting(self, keystates: &KeyStates) -> (Scene, Requests) {
        let reqs = Requests::new()
            .push_imgrq_xy(ImgResID::StageBamboo, 0.0, 0.0, false)
            .push_request(self.chara_1p.get_imgrqs())
            .push_request(Request::Reverse(true))
            .push_request(self.chara_2p.get_imgrqs())
            .push_request(Request::Reverse(false));
        let chara_1p = self.chara_1p.update(keystates.z == 1);
        let chara_2p = self.chara_2p.update(keystates.l == 1);
        println!("Battle!");
        (
            Scene::Game(Self {
                start: self.start,
                count: self.count + 1,
                state: GameState::Shooting,
                chara_1p,
                chara_2p,
            }),
            reqs,
        )
    }
}
