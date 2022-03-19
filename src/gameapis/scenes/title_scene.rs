use super::*;

pub struct TitleScene {}

impl TitleScene {
    pub fn new() -> Result<Box<dyn Scene>, String> {
        Ok(Box::new(TitleScene {}))
    }
}

impl Scene for TitleScene {
    fn update(self: Box<Self>) -> Result<Box<dyn Scene>, String> {
        println!("update title");
        Ok(game_scene::GameScene::new()?)
    }
}
