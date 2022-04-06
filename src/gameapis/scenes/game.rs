use super::super::{
    character::*,
    dialogue::*,
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
#[derive(PartialEq)]
pub enum Mode {
    Story(u32),
}

/// A enum for identifying stage.
pub enum Stage {
    Bamboo,
}

/// A enum for classfying state.
pub enum State {
    Titling,
    Talking,
    Waiting,
    Shooting,
    Ending,
}

pub struct GameScene {
    pub stage: Stage,
    pub start: u32,
    pub mode: Mode,
    pub winner: u32,
    pub state: State,
    pub chara_1p: Character,
    pub chara_2p: Character,
    pub count: u32,
}

impl GameScene {
    /// Constructor.
    pub fn new() -> Scene {
        let rnd: u32 = rand::prelude::random();
        Scene::Game(Self {
            stage: Stage::Bamboo,
            start: (rnd % 300) + 120,
            mode: Mode::Story(1),
            winner: 0,
            state: State::Titling,
            chara_1p: Character::new(CharaID::Udonge),
            chara_2p: Character::new(CharaID::Tei),
            count: 0,
        })
    }
    /// Update game scene. Return the next state and requests.
    pub fn update(self, keystates: &KeyStates, dialogue: &Dialogue) -> (Scene, Requests) {
        let reqs = Requests::new()
            .push_imgrq(ImageRequest::new(get_bg_imgresid(&self.stage)))
            .push_request(get_mask(&self.state))
            .push_imgrq(self.chara_1p.get_imgrq())
            .push_request(Request::Reverse(true))
            .push_imgrq(self.chara_2p.get_imgrq())
            .push_request(Request::Reverse(false))
            .join(get_ui(
                &self.state,
                &self.stage,
                &self.mode,
                &self.count,
                dialogue,
            ));
        let next = self.get_next_scene(keystates, dialogue);
        (next, reqs)
    }
    /// A function to get next scene.
    fn get_next_scene(self, keystates: &KeyStates, dialogue: &Dialogue) -> Scene {
        let input = get_input(&self.state, keystates);
        let count_n = update_count(&self.state, &self.count, keystates);
        let chara_1p = self.chara_1p.update(input.shot_1p);
        let chara_2p = self.chara_2p.update(input.shot_2p);
        let (winner, mode, state, count) = match self.state {
            State::Titling if count_n >= 180 => (None, None, Some(State::Talking), Some(0)),
            State::Talking if count_n == dialogue.get_dialogue_len(&self.mode) as u32 => {
                (None, None, Some(State::Waiting), Some(0))
            }
            State::Waiting if count_n >= self.start => {
                (None, None, Some(State::Shooting), Some(0))
            }
            State::Waiting if !(!chara_1p.is_shot() && !chara_2p.is_shot()) => {
                (Some(2), None, Some(State::Ending), Some(0)) //
            }
            State::Shooting if !(!chara_1p.is_shot() && !chara_2p.is_shot()) => {
                (Some(1), None, Some(State::Ending), Some(0)) //
            }
            _ => (None, None, None, None),
        };
        Scene::Game(Self {
            stage: self.stage,
            start: self.start,
            mode: mode.unwrap_or(self.mode),
            winner: winner.unwrap_or(self.winner),
            state: state.unwrap_or(self.state),
            chara_1p,
            chara_2p,
            count: count.unwrap_or(count_n),
        })
    }
}

/// A function to update count each state.
fn update_count(state: &State, count: &u32, keystates: &KeyStates) -> u32 {
    match state {
        State::Talking => count + indicator_bool(keystates.z == 1 || keystates.l == 1),
        _ => count + 1,
    }
}
/// A function to get player or cpu input information in each state.
fn get_input(state: &State, keystates: &KeyStates) -> InputInfo {
    match state {
        State::Waiting => InputInfo {
            shot_1p: keystates.z == 1,
            shot_2p: keystates.l == 1,
        },
        State::Shooting => InputInfo {
            shot_1p: keystates.z == 1,
            shot_2p: keystates.l == 1,
        },
        _ => InputInfo {
            shot_1p: false,
            shot_2p: false,
        },
    }
}
/// A function to get mask if it needs.
fn get_mask(state: &State) -> Request {
    match state {
        State::Waiting => Request::Image(ImageRequest::new(ImgResID::Black).alph(0.5)),
        State::Shooting => Request::Image(ImageRequest::new(ImgResID::Red).alph(0.2)),
        _ => Request::NoRequest,
    }
}
/// A function to get dialogue text.
fn get_ui(state: &State, stage: &Stage, mode: &Mode, count: &u32, dialogue: &Dialogue) -> Requests {
    match state {
        State::Titling => {
            if count >= &20 {
                Requests::new().push_imgrq(
                    ImageRequest::new(get_title_imgresid(stage))
                        .lt(640.0, 300.0)
                        .cntr(true)
                        .alph(1.0 - min(max((count.clone() as f32 - 80.0) / 30.0, 0.0), 1.0)),
                )
            } else {
                Requests::new()
            }
        }
        State::Talking => get_dialogue_requests(dialogue, mode, count),
        _ => Requests::new(),
    }
}
/// A function to get dialogue requests.
fn get_dialogue_requests(dialogue: &Dialogue, mode: &Mode, count: &u32) -> Requests {
    if let Some((s, b)) = dialogue.get_dialogue(mode, count) {
        Requests::new()
            .push_request(Request::Reverse(b))
            .push_imgrq(
                ImageRequest::new(ImgResID::SpeechBubble)
                    .lt(640.0, 200.0)
                    .cntr(true),
            )
            .push_request(Request::Reverse(false))
            .push_txtrq(
                TextRequest::new(s)
                    .ltrb(0.0, 170.0, 1280.0, 720.0)
                    .rgba(0.0, 0.0, 0.0, 1.0)
                    .set_size(40.0)
                    .set_align(1),
            )
    } else {
        Requests::new()
    }
}
/// A function to get stage bg ImgResID based on Stage.
fn get_bg_imgresid(stage: &Stage) -> ImgResID {
    match stage {
        Stage::Bamboo => ImgResID::StageBamboo,
    }
}
/// A function to get stage title ImgResID based on Stage.
fn get_title_imgresid(stage: &Stage) -> ImgResID {
    match stage {
        Stage::Bamboo => ImgResID::StageTitleBamboo,
    }
}
