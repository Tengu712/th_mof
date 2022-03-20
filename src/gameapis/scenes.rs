use super::resources::*;

/// A struct that has a function field that updates each scene and common fields that all scene need.
pub struct Scene {
    updater: fn(Self) -> (Self, Vec<Request>),
}

impl Scene {
    /// Constructor for user.
    pub fn new() -> Self {
        Self {
            updater: update_title,
        }
    }
    /// Update scene.
    pub fn update(self) -> (Self, Vec<Request>) {
        (self.updater)(self)
    }
}

/// Private function. Update title scene.
fn update_title(scene: Scene) -> (Scene, Vec<Request>) {
    let mut reqs = Vec::new();
    reqs.push(create_imgreq_wh(
        String::from("a"),
        0.0,
        0.0,
        1280.0,
        720.0,
        false,
    ));
    (scene, reqs)
}
