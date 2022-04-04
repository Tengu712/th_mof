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
    pub fn update_ending(self, keystates: &KeyStates) -> (Scene, Requests) {
        let reqs = Requests::new()
            .push_imgrq(ImageRequest::new(self.get_imgresid()))
            .push_imgrq(
                ImageRequest::new(self.chara_1p.get_imgresid())
                    .wh(512.0, 720.0)
                    .uv(0.0, 0.0),
            )
            .push_request(Request::Reverse(true))
            .push_imgrq(
                ImageRequest::new(self.chara_2p.get_imgresid())
                    .wh(512.0, 720.0)
                    .uv(0.0, 0.0),
            )
            .push_request(Request::Reverse(false));
        if keystates.z == 1 || keystates.l == 1 {
            (TitleScene::new(), reqs)
        } else {
            (
                Scene::Game(Self {
                    stage: self.stage,
                    mode: self.mode,
                    start: self.start,
                    winner: self.winner,
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
