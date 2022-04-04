use super::*;

impl GameScene {
    ///
    pub fn update_talking(self, keystates: &KeyStates) -> (Scene, Requests) {
        let reqs = Requests::new()
            .push_imgrq_xy(ImgResID::StageBamboo, 0.0, 0.0, false)
            .push_request(self.chara_1p.get_imgrqs())
            .push_request(Request::Reverse(true))
            .push_request(self.chara_2p.get_imgrqs())
            .push_request(Request::Reverse(false));
        let (state, count) = if keystates.z == 1 || keystates.l == 1 {
            (GameState::Waiting, 0)
        } else {
            (GameState::Talking, self.count)
        };
        (
            Scene::Game(Self {
                start: self.start,
                count,
                state,
                chara_1p: self.chara_1p,
                chara_2p: self.chara_2p,
            }),
            reqs,
        )
    }
}
