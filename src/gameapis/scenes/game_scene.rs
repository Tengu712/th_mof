use super::*;

pub struct GameScene {}

impl GameScene {
    pub fn new() -> Result<Box<dyn Scene>, String> {
        Ok(Box::new(GameScene {}))
    }
}

impl Scene for GameScene {
    fn update(self: Box<Self>) -> Result<Box<dyn Scene>, String> {
        println!("update game");
        Ok(self)
    }
}
