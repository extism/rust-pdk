type Handle = u64;

#[repr(u32)]
pub enum Stream {
    Input = 0,
    Output = 1,
}

#[link(wasm_import_module = "extism:host/env")]
extern "C" {
    pub fn read(stream: Stream, handle: Handle) -> i64;
    pub fn write(stream: Stream, handle: Handle) -> i64;
    pub fn close(stream: Stream);
    pub fn bytes_remaining(stream: Stream) -> i64;

    pub fn stack_push();
    pub fn stack_pop();

    pub fn error(handle: Handle) -> !;
    pub fn config_length(key: Handle) -> i64;
    pub fn config_read(key: Handle, value: Handle) -> i64;
    pub fn log(level: i32, value: Handle);
    pub fn http_request(req: Handle, body: Handle) -> i64;
    pub fn http_body(value: Handle) -> i64;
    pub fn http_status_code() -> i32;
}

pub struct Frame {}

impl Frame {
    pub fn new() -> Self {
        unsafe { stack_push() };
        Frame {}
    }
}

impl Default for Frame {
    fn default() -> Self {
        Frame::new()
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe { stack_pop() }
    }
}
