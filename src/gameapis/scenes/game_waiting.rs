use super::{
    super::{
        input::*,
        requests::{imagerequest::*, *},
        resource::*,
        scenes::*,
    },
    game::*,
};

impl GameScene {
    /// Backgound, black mask, characters.
    /// Player can start shooting. If it shoots during the state, it loses.
    pub fn update_waiting(self, keystates: &KeyStates) -> (Scene, Requests) {
        let reqs = Requests::new()
            .push_imgrq(ImageRequest::new(self.get_imgresid()))
            .push_imgrq(ImageRequest::new(ImgResID::Black).alph(0.5))
            .push_imgrq(self.chara_1p.get_imgrq())
            .push_request(Request::Reverse(true))
            .push_imgrq(self.chara_2p.get_imgrq())
            .push_request(Request::Reverse(false));
        let chara_1p = self.chara_1p.update(keystates.z == 1);
        let chara_2p = self.chara_2p.update(keystates.l == 1);
        let (state, winner, count) = if self.count == self.start {
            (GameState::Shooting, 0, 0)
        } else if chara_1p.is_shot() || chara_2p.is_shot() {
            (GameState::Ending, 2, 0)
        } else {
            (GameState::Waiting, 0, self.count + 1)
        };
        (
            Scene::Game(Self {
                stage: self.stage,
                mode: self.mode,
                start: self.start,
                winner,
                count,
                state,
                chara_1p,
                chara_2p,
            }),
            reqs,
        )
    }
}
