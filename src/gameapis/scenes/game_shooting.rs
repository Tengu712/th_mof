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
    /// Background, red mask, characters.
    pub fn update_shooting(self, keystates: &KeyStates) -> (Scene, Requests) {
        let reqs = Requests::new()
            .push_imgrq(ImageRequest::new(self.get_imgresid()))
            .push_imgrq(ImageRequest::new(ImgResID::Red).alph(0.2))
            .push_imgrq(self.chara_1p.get_imgrq())
            .push_request(Request::Reverse(true))
            .push_imgrq(self.chara_2p.get_imgrq())
            .push_request(Request::Reverse(false));
        let chara_1p = self.chara_1p.update(keystates.z == 1);
        let chara_2p = self.chara_2p.update(keystates.l == 1);
        (
            Scene::Game(Self {
                stage: self.stage,
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
