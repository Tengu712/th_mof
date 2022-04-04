use super::*;

pub enum GameState {
    Talking,
    Waiting,
    Shooting,
    Ending,
}

pub struct GameScene {
    pub start: u32,
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
            start: (rnd % 300) + 120,
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
}
