use super::super::{character::*, input::*, requests::*, resource::*, scenes::*};

/// A enum for classfying mode.
pub enum Mode {
    Story(u32),
}

pub enum Stage {
    Bamboo,
}

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
        match self.state {
            GameState::Talking => self.update_talking(keystates),
            GameState::Waiting => self.update_waiting(keystates),
            GameState::Shooting => self.update_shooting(keystates),
            GameState::Ending => self.update_ending(keystates),
        }
    }
    /// A method to get ImgResID based on Stage.
    pub fn get_imgresid(&self) -> ImgResID {
        match self.stage {
            Stage::Bamboo => ImgResID::StageBamboo,
        }
    }
}
