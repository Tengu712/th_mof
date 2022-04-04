use super::{game::*, *};

impl GameScene {
    ///
    pub fn update_ending(self, keystates: &KeyStates) -> (Scene, Requests) {
        let reqs = Requests::new()
        .push_imgrq_xy(ImgResID::StageBamboo, 0.0, 0.0, false)
        .push_request(self.chara_1p.get_imgrqs())
        .push_request(Request::Reverse(true))
        .push_request(self.chara_2p.get_imgrqs())
        .push_request(Request::Reverse(false));
        if keystates.z == 1 || keystates.l == 1 {
            (TitleScene::new(), reqs)
        } else {
            (
                Scene::Game(Self {
                    start: self.start,
                    count: 0,
                    state: GameState::Ending,
                    chara_1p: self.chara_1p,
                    chara_2p: self.chara_2p,
                }),
                reqs,
            )
        }
    }
}
