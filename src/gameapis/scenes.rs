/// A struct that has a function field that updates each scene and common fields that all scene need.
pub struct Scene {
    updater: fn(Self) -> Result<Self, String>,
}

impl Scene {
    /// Constructor for user.
    pub fn new() -> Result<Self, String> {
        Ok(Scene {
            updater: update_title,
        })
    }
    /// Update scene.
    pub fn update(self) -> Result<Self, String> {
        (self.updater)(self)
    }
}

/// Private function. Update title scene.
fn update_title(_: Scene) -> Result<Scene, String> {
    Ok(Scene {
        updater: update_game,
    })
}

/// Private function. Update game scene.
fn update_game(scene: Scene) -> Result<Scene, String> {
    Ok(Scene {
        updater: scene.updater,
    })
}
