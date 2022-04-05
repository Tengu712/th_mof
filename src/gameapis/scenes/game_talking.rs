use super::{
    super::{
        input::*,
        requests::{imagerequest::*, textrequest::*, *},
        resource::*,
        scenes::*,
        *,
    },
    game::*,
};

impl GameScene {
    /// Background, characters.
    /// If dialogue is none, go to waiting state.
    pub fn update_talking(self, keystates: &KeyStates) -> (Scene, Requests) {
        let mut reqs = Requests::new()
            .push_imgrq(ImageRequest::new(self.get_imgresid()))
            .push_imgrq(self.chara_1p.get_imgrq())
            .push_request(Request::Reverse(true))
            .push_imgrq(self.chara_2p.get_imgrq())
            .push_request(Request::Reverse(false));
        let count = self.count + indicator_bool(keystates.z == 1 || keystates.l == 1);
        let text = self.get_dialogue();
        let state = match text {
            Some(n) => {
                reqs = reqs
                    .push_imgrq(
                        ImageRequest::new(ImgResID::SpeechBubble)
                            .lt(640.0, 180.0)
                            .cntr(true),
                    )
                    .push_txtrq(
                        TextRequest::new(n)
                            .ltrb(0.0, 160.0, 1280.0, 720.0)
                            .rgba(0.0, 0.0, 0.0, 1.0)
                            .set_size(50.0)
                            .set_align(1),
                    );
                GameState::Talking
            }
            None => GameState::Waiting,
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
    fn get_dialogue(&self) -> Option<&str> {
        match self.mode {
            Mode::Story(n) => match n {
                1 => match self.count {
                    0 => Some("あいうえお"),
                    _ => None,
                },
                _ => None,
            },
        }
    }
}
