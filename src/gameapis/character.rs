use super::{requests::imagerequest::*, resource::*};

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
    pub fn update(self, shoot: bool) -> Self {
        let end = match self.id {
            CharaID::Udonge => 13,
            CharaID::Tei => 13,
        };
        let shoot_count = if self.shoot_count > 0 && self.shoot_count < end {
            self.shoot_count + 1
        } else if shoot {
            1
        } else {
            0
        };
        Self {
            id: self.id,
            shoot_count,
        }
    }
    /// A method to get ImgResID based on CharaID.
    pub fn get_imgresid(&self) -> ImgResID {
        match self.id {
            CharaID::Udonge => ImgResID::Udonge,
            CharaID::Tei => ImgResID::Tei,
        }
    }
    /// A method
    pub fn get_imgrq(&self) -> ImageRequest {
        let imgrq = ImageRequest::new(self.get_imgresid())
            .wh(512.0, 720.0)
            .uv(0.0, 0.0);
        match self.id {
            CharaID::Udonge if self.shoot_count > 0 => imgrq.uv(512.0, 0.0),
            CharaID::Tei if self.shoot_count > 0 => imgrq.uv(512.0, 0.0),
            _ => imgrq,
        }
    }
}
