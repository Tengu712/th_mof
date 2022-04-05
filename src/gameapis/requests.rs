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
        match request {
            Request::NoRequest => self,
            _ if self.count >= self.array.len() => self,
            _ => {
                let mut array_mut = self.array;
                array_mut[self.count] = request;
                Self {
                    count: self.count + 1,
                    array: array_mut,
                }
            }
        }
    }
    /// Join two requests.
    pub fn join(self, requests: Requests) -> Self {
        let mut origin = self;
        for i in requests.get_array().iter() {
            origin = origin.push_request(*i);
        }
        origin
    }
}
