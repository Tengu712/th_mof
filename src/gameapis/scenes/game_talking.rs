use super::{
    super::{
        input::*,
        requests::{imagerequest::*, *},
        scenes::*,
        *,
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
        let count = self.count + indicator_bool(keystates.z == 1 || keystates.l == 1);
        let state = if self.get_dialogue() {
            GameState::Waiting
        } else {
            GameState::Talking
        };
        (
            Scene::Game(Self {
                stage: self.stage,
                mode: self.mode,
                start: self.start,
                winner: 0,
                count,
                state,
                chara_1p: self.chara_1p,
                chara_2p: self.chara_2p,
            }),
            reqs,
        )
    }
    /// Get dialogue.
    fn get_dialogue(&self) -> bool {
        match self.mode {
            Mode::Story(n) => match n {
                1 => match self.count {
                    _ => true,
                },
                _ => true,
            },
        }
    }
}
