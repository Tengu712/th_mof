use super::super::{
    character::*,
    input::*,
    requests::{imagerequest::*, textrequest::*, *},
    resource::*,
    scenes::*,
    *,
};

/// A private struct for management of player or CPU inputs.
struct InputInfo {
    shot_1p: bool,
    shot_2p: bool,
}

/// A enum for classfying mode.
pub enum Mode {
    Story(u32),
}

/// A enum for identifying stage.
pub enum Stage {
    Bamboo,
}

/// A enum for classfying state.
pub enum GameState {
    Talking,
    Waiting,
    Shooting,
    Ending,
}

pub struct GameScene {
    pub stage: Stage,
    pub mode: Mode,
    pub start: u32,
    pub winner: u32,
    pub count: u32,
    pub state: GameState,
    pub chara_1p: Character,
    pub chara_2p: Character,
}

impl GameScene {
    /// Constructor.
    pub fn new() -> Scene {
        let rnd: u32 = rand::prelude::random();
        Scene::Game(Self {
            stage: Stage::Bamboo,
            mode: Mode::Story(1),
            start: (rnd % 300) + 120,
            winner: 0,
            count: 0,
            state: GameState::Talking,
            chara_1p: Character::new(CharaID::Udonge),
            chara_2p: Character::new(CharaID::Tei),
        })
    }
    /// Update game scene. Return the next state and requests.
    pub fn update(self, keystates: &KeyStates) -> (Scene, Requests) {
        let count = self.update_count(keystates);
        let input = self.get_input(keystates);
        let chara_1p = self.chara_1p.update(input.shot_1p);
        let chara_2p = self.chara_2p.update(input.shot_2p);
        let mask = self.get_mask();
        let text = self.get_text();
        let reqs = Requests::new()
            .push_imgrq(ImageRequest::new(self.get_bg_imgresid()))
            .push_request(mask)
            .push_imgrq(chara_1p.get_imgrq())
            .push_request(Request::Reverse(true))
            .push_imgrq(chara_2p.get_imgrq())
            .push_request(Request::Reverse(false))
            .join(text);
        (
            Scene::Game(Self {
                stage: self.stage,
                mode: self.mode,
                start: self.start,
                winner: 0,
                count,
                state: GameState::Talking,
                chara_1p,
                chara_2p,
            }),
            reqs,
        )
    }
    /// A method to update count each state.
    fn update_count(&self, keystates: &KeyStates) -> u32 {
        match self.state {
            GameState::Talking => self.count + indicator_bool(keystates.z == 1 || keystates.l == 1),
            _ => self.count + 1,
        }
    }
    /// Get input information in each state.
    fn get_input(&self, keystates: &KeyStates) -> InputInfo {
        match self.state {
            GameState::Waiting => InputInfo {
                shot_1p: keystates.z == 1,
                shot_2p: keystates.l == 1,
            },
            _ => InputInfo {
                shot_1p: false,
                shot_2p: false,
            },
        }
    }
    /// A method to get mask if it needs.
    fn get_mask(&self) -> Request {
        match self.state {
            GameState::Waiting => Request::Image(ImageRequest::new(ImgResID::Black).alph(0.5)),
            GameState::Shooting => Request::Image(ImageRequest::new(ImgResID::Red).alph(0.2)),
            _ => Request::NoRequest,
        }
    }
    /// A method to get dialogue text.
    fn get_text(&self) -> Requests {
        match self.state {
            GameState::Talking => match self.get_dialogue() {
                Some((s, b)) => Requests::new()
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
                    ),
                None => Requests::new(),
            },
            _ => Requests::new(),
        }
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
    /// A method to get ImgResID based on Stage.
    fn get_bg_imgresid(&self) -> ImgResID {
        match self.stage {
            Stage::Bamboo => ImgResID::StageBamboo,
        }
    }
}
