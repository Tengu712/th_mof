use super::resource::*;

/// A enum for identifing characters.
pub enum CharaID {
    Udonge,
    Tei,
}

/// A struct for keeping character information.
pub struct Character {
    id: CharaID,
    shoot_count: u32,
}

impl Character {
    /// Constructor.
    pub fn new(id: CharaID) -> Self {
        Self { id, shoot_count: 0 }
    }
    /// A method for abstracting updating shoot count.
    pub fn update(self, keystate: i16) -> Self {
        let end = match self.id {
            CharaID::Udonge => 13,
            CharaID::Tei => 13,
        };
        let shoot_count = if self.shoot_count > 0 && self.shoot_count < end {
            self.shoot_count + 1
        } else if keystate == 1 {
            1
        } else {
            0
        };
        Self {
            id: self.id,
            shoot_count,
        }
    }
    /// A method
    pub fn get_drawinfo(&self) -> (ImgResID, f32, f32) {
        if self.shoot_count > 0 {
            match self.id {
                CharaID::Udonge => (ImgResID::Udonge, 512.0, 0.0),
                CharaID::Tei => (ImgResID::Tei, 512.0, 0.0),
            }
        } else {
            match self.id {
                CharaID::Udonge => (ImgResID::Udonge, 0.0, 0.0),
                CharaID::Tei => (ImgResID::Tei, 0.0, 0.0),
            }
        }
    }
}
