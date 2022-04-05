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
    /// Background, characters, speech bubble, text.
    /// If dialogue is none, go to waiting state.
    pub fn update_talking(self, keystates: &KeyStates) -> (Scene, Requests) {
        let reqs = Requests::new()
            .push_imgrq(ImageRequest::new(self.get_imgresid()))
            .push_imgrq(self.chara_1p.get_imgrq())
            .push_request(Request::Reverse(true))
            .push_imgrq(self.chara_2p.get_imgrq())
            .push_request(Request::Reverse(false));
        let count = self.count + indicator_bool(keystates.z == 1 || keystates.l == 1);
        let (state, reqs) = match self.get_dialogue() {
            Some((s, b)) => {
                let reqs = reqs
                    .push_request(Request::Reverse(b))
                    .push_imgrq(
                        ImageRequest::new(ImgResID::SpeechBubble)
                            .lt(640.0, 180.0)
                            .cntr(true),
                    )
                    .push_request(Request::Reverse(false))
                    .push_txtrq(
                        TextRequest::new(s)
                            .ltrb(0.0, 160.0, 1280.0, 720.0)
                            .rgba(0.0, 0.0, 0.0, 1.0)
                            .set_size(50.0)
                            .set_align(1),
                    );
                (GameState::Talking, reqs)
            }
            None => (GameState::Waiting, reqs),
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
    /// Get dialogue. Return (&str, bool). The bool is true, 2p is speaking.
    fn get_dialogue(&self) -> Option<(&str, bool)> {
        match self.mode {
            Mode::Story(n) => match n {
                1 => match self.count {
                    0 => Some(("鈴仙・U・イナバ", false)),
                    1 => Some(("因幡てゐ", true)),
                    _ => None,
                },
                _ => None,
            },
        }
    }
}
