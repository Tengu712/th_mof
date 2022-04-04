use super::{super::resource::ImgResID, *};

/// A struct that's entity of drawing image request.
#[derive(Clone, Copy)]
pub struct ImageRequest {
    pub key: ImgResID,
    pub left: f32,
    pub top: f32,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub uv_left: f32,
    pub uv_top: f32,
    pub uv_width: Option<f32>,
    pub uv_height: Option<f32>,
    pub alpha: f32,
    pub center: bool,
}

impl ImageRequest {
    /// Constructor.
    pub fn new(key: ImgResID) -> Self {
        Self {
            key,
            left: 0.0,
            top: 0.0,
            width: None,
            height: None,
            uv_left: 0.0,
            uv_top: 0.0,
            uv_width: None,
            uv_height: None,
            alpha: 1.0,
            center: false,
        }
    }
    /// A method to set left and top.
    pub fn lt(self, left: f32, top: f32) -> Self {
        let mut cloned = self.clone();
        cloned.left = left;
        cloned.top = top;
        cloned
    }
    /// A method to set width and height.
    pub fn wh(self, width: f32, height: f32) -> Self {
        let mut cloned = self.clone();
        cloned.width = Some(width);
        cloned.height = Some(height);
        cloned
    }
    /// A method to set uv. 
    /// uv_width and uv_height will be same as width and height.
    pub fn uv(self, uv_left: f32, uv_top: f32) -> Self {
        let mut cloned = self.clone();
        cloned.uv_left = uv_left;
        cloned.uv_top = uv_top;
        cloned.uv_width = cloned.width;
        cloned.uv_height = cloned.height;
        cloned
    }
    /// A method to set alpha.
    pub fn alph(self, alpha: f32) -> Self {
        let mut cloned = self.clone();
        cloned.alpha = alpha;
        cloned
    }
    /// A method to set center.
    pub fn cntr(self, center: bool) -> Self {
        let mut cloned = self.clone();
        cloned.center = center;
        cloned
    }
}

impl Requests {
    /// Push image request with wrapping ImageRequest struct.
    pub fn push_imgrq(self, imgrq: ImageRequest) -> Self {
        self.push_request(Request::Image(imgrq))
    }
}
