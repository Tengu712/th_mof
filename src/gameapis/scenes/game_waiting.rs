use super::*;

impl GameScene {
    ///
    pub fn update_waiting(self, keystates: &KeyStates) -> (Scene, Requests) {
        let reqs = Requests::new()
            .push_imgrq_xy(ImgResID::StageBamboo, 0.0, 0.0, false)
            .push_request(self.chara_1p.get_imgrqs())
            .push_request(Request::Reverse(true))
            .push_request(self.chara_2p.get_imgrqs())
            .push_request(Request::Reverse(false));
        let (state, count) = if self.count == self.start {
            println!("bang! {}  {}", self.count, self.start);
            (GameState::Shooting, 0)
        } else if keystates.z == 1 || keystates.l == 1 {
            println!("otetuki");
            (GameState::Ending, 0)
        } else {
            (GameState::Waiting, self.count + 1)
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
