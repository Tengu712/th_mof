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
    pub center: bool,
}

impl Requests {
    /// Push image request simply.
    pub fn push_imgrq(self, key: ImgResID, left: f32, top: f32, center: bool) -> Self {
        self.push_request(Request::Image(ImageRequest {
            key,
            left,
            top,
            width: None,
            height: None,
            uv_left: 0.0,
            uv_top: 0.0,
            uv_width: None,
            uv_height: None,
            center,
        }))
    }

    /// Push image request with width and height.
    pub fn push_imgrq_wh(
        self,
        key: ImgResID,
        left: f32,
        top: f32,
        width: f32,
        height: f32,
        center: bool,
    ) -> Self {
        self.push_request(Request::Image(ImageRequest {
            key,
            left,
            top,
            width: Some(width),
            height: Some(height),
            uv_left: 0.0,
            uv_top: 0.0,
            uv_width: None,
            uv_height: None,
            center,
        }))
    }

    /// Push image request with width and height.
    pub fn push_imgrq_whuv(
        self,
        key: ImgResID,
        left: f32,
        top: f32,
        width: f32,
        height: f32,
        uv_left: f32,
        uv_top: f32,
        uv_width: f32,
        uv_height: f32,
        center: bool,
    ) -> Self {
        self.push_request(Request::Image(ImageRequest {
            key,
            left,
            top,
            width: Some(width),
            height: Some(height),
            uv_left: uv_left,
            uv_top: uv_top,
            uv_width: Some(uv_width),
            uv_height: Some(uv_height),
            center,
        }))
    }
}
