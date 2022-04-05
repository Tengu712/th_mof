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
        let mut self_mut = self;
        self_mut.left = left;
        self_mut.top = top;
        self_mut
    }
    /// A method to set width and height.
    pub fn wh(self, width: f32, height: f32) -> Self {
        let mut self_mut = self;
        self_mut.width = Some(width);
        self_mut.height = Some(height);
        self_mut
    }
    /// A method to set uv. 
    /// uv_width and uv_height will be same as width and height.
    pub fn uv(self, uv_left: f32, uv_top: f32) -> Self {
        let mut self_mut = self;
        self_mut.uv_left = uv_left;
        self_mut.uv_top = uv_top;
        self_mut.uv_width = self_mut.width;
        self_mut.uv_height = self_mut.height;
        self_mut
    }
    /// A method to set alpha.
    pub fn alph(self, alpha: f32) -> Self {
        let mut self_mut = self;
        self_mut.alpha = alpha;
        self_mut
    }
    /// A method to set center.
    pub fn cntr(self, center: bool) -> Self {
        let mut self_mut = self;
        self_mut.center = center;
        self_mut
    }
}

impl Requests {
    /// Push image request with wrapping ImageRequest struct.
    pub fn push_imgrq(self, imgrq: ImageRequest) -> Self {
        self.push_request(Request::Image(imgrq))
    }
}
