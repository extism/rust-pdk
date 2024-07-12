type Handle = u64;

#[link(wasm_import_module = "extism:host/env")]
extern "C" {
    pub fn input_read(handle: Handle) -> i64;
    pub fn output_write(handle: Handle);
    pub fn error(handle: Handle) -> !;
    pub fn config_length(key: Handle) -> i64;
    pub fn config_read(key: Handle, value: Handle) -> i64;
    pub fn log(level: i32, value: Handle);
    // pub fn var_get(offs: u64) -> u64;
    // pub fn var_set(offs: u64, offs1: u64);
    pub fn http_request(req: Handle, body: Handle) -> i64;
    pub fn http_body(value: Handle) -> i64;
    pub fn http_status_code() -> i32;
}
