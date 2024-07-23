type Handle = u64;

#[link(wasm_import_module = "extism:host/env")]
extern "C" {
    pub fn input_read(handle: Handle) -> i64;
    pub fn output_write(handle: Handle) -> i64;

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
