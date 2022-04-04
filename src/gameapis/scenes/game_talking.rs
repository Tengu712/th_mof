use super::{
    super::{
        input::*,
        requests::{imagerequest::*, *},
        scenes::*,
    },
    game::*,
};

impl GameScene {
    /// Background, characters. 
    pub fn update_talking(self, keystates: &KeyStates) -> (Scene, Requests) {
        let reqs = Requests::new()
            .push_imgrq(ImageRequest::new(self.get_imgresid()))
            .push_imgrq(self.chara_1p.get_imgrq())
            .push_request(Request::Reverse(true))
            .push_imgrq(self.chara_2p.get_imgrq())
            .push_request(Request::Reverse(false));
        let (state, count) = if keystates.z == 1 || keystates.l == 1 {
            (GameState::Waiting, 0)
        } else {
            (GameState::Talking, self.count)
        };
        (
            Scene::Game(Self {
                stage: self.stage,
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
