pub mod game_scene;
pub mod title_scene;

/// A trait for scenes.
pub trait Scene {
    /// Update scene.
    fn update(self: Box<Self>) -> Result<Box<dyn Scene>, String>;
}
