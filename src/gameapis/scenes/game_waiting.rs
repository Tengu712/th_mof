use super::{
    super::{
        input::*,
        requests::{imagerequest::*, *},
        scenes::*,
    },
    game::*,
};

impl GameScene {
    ///
    pub fn update_waiting(self, keystates: &KeyStates) -> (Scene, Requests) {
        let reqs = Requests::new()
            .push_imgrq(ImageRequest::new(self.get_imgresid()))
            .push_imgrq(self.chara_1p.get_imgrq())
            .push_request(Request::Reverse(true))
            .push_imgrq(self.chara_2p.get_imgrq())
            .push_request(Request::Reverse(false));
        let chara_1p = self.chara_1p.update(keystates.z == 1);
        let chara_2p = self.chara_2p.update(keystates.l == 1);
        let (state, count) = if self.count == self.start {
            println!("bang! {}  {}", self.count, self.start);
            (GameState::Shooting, 0)
        } else {
            (GameState::Waiting, self.count + 1)
        };
        (
            Scene::Game(Self {
                stage: self.stage,
                start: self.start,
                count,
                state,
                chara_1p,
                chara_2p,
            }),
            reqs,
        )
    }
}
