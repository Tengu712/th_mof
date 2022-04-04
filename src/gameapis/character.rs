use super::{
    requests::{imagerequest::*, *},
    resource::*,
};

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
    /// A method to get ImgResID based on CharaID.
    fn get_imgresid(&self) -> ImgResID {
        match self.id {
            CharaID::Udonge => ImgResID::Udonge,
            CharaID::Tei => ImgResID::Tei,
        }
    }
    /// A method
    pub fn get_imgrqs(&self) -> Request {
        let mut imgrq = ImageRequest {
            key: self.get_imgresid(),
            left: 0.0,
            top: 0.0,
            width: Some(512.0),
            height: Some(720.0),
            uv_left: 0.0,
            uv_top: 0.0,
            uv_width: Some(512.0),
            uv_height: Some(720.0),
            center: false,
        };
        match self.id {
            CharaID::Udonge if self.shoot_count > 0 => imgrq.uv_left = 512.0,
            CharaID::Tei if self.shoot_count > 0 => imgrq.uv_left = 512.0,
            _ => (),
        }
        Request::Image(imgrq)
    }
}
