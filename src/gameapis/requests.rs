pub mod imagerequest;
pub mod textrequest;

use imagerequest::*;
use textrequest::*;

/// A enum for request of drawing image or.
#[derive(Clone, Copy)]
pub enum Request {
    NoRequest,
    Reverse(bool),
    Image(ImageRequest),
    Text(TextRequest),
}

const NUM_REQUESTS: usize = 20;

/// A struct for drawing image request or that's messaged from scene to mainloop.
/// All method is reserved immutable self.
pub struct Requests {
    count: usize,
    array: [Request; NUM_REQUESTS],
}

impl Requests {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            count: 0,
            array: [Request::NoRequest; NUM_REQUESTS],
        }
    }
    /// Get request array and drop.
    pub fn get_array(self) -> [Request; NUM_REQUESTS] {
        self.array
    }
    /// Push request to array at count and incliment count.
    /// If count is bigger than length of array, nothing happens.
    pub fn push_request(self, request: Request) -> Self {
        if self.count >= self.array.len() {
            return self;
        }
        let mut cloned = self.array.clone();
        cloned[self.count] = request;
        Self {
            count: self.count + 1,
            array: cloned,
        }
    }
}
