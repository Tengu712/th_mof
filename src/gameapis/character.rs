use super::{requests::imagerequest::*, resource::*, *};

/// A enum for identifing characters.
#[derive(Clone)]
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
    pub fn update(&self, shoot: bool) -> Self {
        let end = self.get_end_of_shot();
        let shoot_count = (self.shoot_count + 1)
            * indicator_u32(self.shoot_count)
            * indicator_bool(self.shoot_count > 0 && self.shoot_count < end)
            + indicator_bool(shoot) * indicator_bool(self.shoot_count == 0);
        Self {
            id: self.id.clone(),
            shoot_count,
        }
    }
    /// A method to get ImageRequest of character.
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
    /// A method to get ImgResID based on CharaID.
    pub fn get_imgresid(&self) -> ImgResID {
        match self.id {
            CharaID::Udonge => ImgResID::Udonge,
            CharaID::Tei => ImgResID::Tei,
        }
    }
    /// A method to get frame of end of shot.
    fn get_end_of_shot(&self) -> u32 {
        match self.id {
            _ => 13,
        }
    }
    /// A method to judge shoot timing.
    pub fn is_shot(&self) -> bool {
        match self.id {
            _ if self.shoot_count == 3 => true,
            _ => false,
        }
    }
}
