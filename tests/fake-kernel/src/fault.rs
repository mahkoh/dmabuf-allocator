#[derive(Copy, Clone)]
pub struct Fault {
    pub request: u32,
    pub occurrence: usize,
    pub errno: i32,
    pub response: bool,
}

impl Fault {
    pub fn new(request: u32) -> Self {
        Self {
            request,
            occurrence: 1,
            errno: libc::EINVAL,
            response: true,
        }
    }

    pub fn nth(mut self, occurrence: usize) -> Self {
        self.occurrence = occurrence;
        self
    }

    pub fn errno(mut self, errno: i32) -> Self {
        self.errno = errno;
        self
    }

    pub fn without_response(mut self) -> Self {
        self.response = false;
        self
    }
}
