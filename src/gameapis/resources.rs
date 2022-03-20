/// A enum for request of drawing image or.
pub enum Request {
    Image(ImageRequest),
}

/// A struct that's entity of drawing image request.
pub struct ImageRequest {
    pub key: String,
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

/// Create ImageRequest as Request simply.
pub fn create_imgreq(key: String, left: f32, top: f32, center: bool) -> Request {
    Request::Image(ImageRequest {
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
    })
}

/// Create ImageRequest as Request with width and height.
pub fn create_imgreq_wh(
    key: String,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
    center: bool,
) -> Request {
    Request::Image(ImageRequest {
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
    })
}

/// Create ImageRequest as Request with uv coodinates.
pub fn create_imgreq_uv(
    key: String,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
    uv_left: f32,
    uv_top: f32,
    uv_width: f32,
    uv_height: f32,
    center: bool,
) -> Request {
    Request::Image(ImageRequest {
        key,
        left,
        top,
        width: Some(width),
        height: Some(height),
        uv_left,
        uv_top,
        uv_width: Some(uv_width),
        uv_height: Some(uv_height),
        center,
    })
}
