use super::*;

const NUM_CHARACTERS: usize = 80;

/// A struct that's entity of drawing text request.
#[derive(Clone, Copy)]
pub struct TextRequest {
    pub text: [u8; NUM_CHARACTERS],
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub size: f32,
    pub alignment: u32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl TextRequest {
    /// Constructor.
    pub fn new(text_str: String) -> Self {
        let txt_arr = text_str.as_bytes();
        let mut text = [0u8; NUM_CHARACTERS];
        for i in 0..NUM_CHARACTERS {
            if i >= txt_arr.len() {
                break;
            }
            text[i] = txt_arr[i];
        }
        Self {
            text,
            left: 0.0,
            top: 0.0,
            right: 0.0,
            bottom: 0.0,
            size: 32.0,
            alignment: 0,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
    /// A method to set left, top, right and bottom.
    pub fn ltrb(self, left: f32, top: f32, right: f32, bottom: f32) -> Self {
        let mut self_mut = self;
        self_mut.left = left;
        self_mut.top = top;
        self_mut.right = right;
        self_mut.bottom = bottom;
        self_mut
    }
    /// A method to set font size.
    pub fn set_size(self, size: f32) -> Self {
        let mut self_mut = self;
        self_mut.size = size;
        self_mut
    }
    /// A method to set alignment.
    pub fn set_align(self, alignment: u32) -> Self {
        let mut self_mut = self;
        self_mut.alignment = alignment;
        self_mut
    }
    /// A method to set literal color.
    pub fn rgba(self, r: f32, g: f32, b: f32, a: f32) -> Self {
        let mut self_mut = self;
        self_mut.r = r;
        self_mut.g = g;
        self_mut.b = b;
        self_mut.a = a;
        self_mut
    }
}

impl Requests {
    /// Push text request with wrapping TextRequest struct.
    pub fn push_txtrq(self, txtrq: TextRequest) -> Self {
        self.push_request(Request::Text(txtrq))
    }
}
