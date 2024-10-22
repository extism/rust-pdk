#[link(wasm_import_module = "extism:host/env")]
extern "C" {
    pub fn input_length() -> u64;
    pub fn input_load_u8(offs: u64) -> u8;
    pub fn input_load_u64(offs: u64) -> u64;
    pub fn length(offs: u64) -> u64;
    pub fn length_unsafe(offs: u64) -> u64;
    pub fn alloc(length: u64) -> u64;
    pub fn free(offs: u64);
    pub fn output_set(offs: u64, length: u64);
    pub fn error_set(offs: u64);
    pub fn store_u8(offs: u64, data: u8);
    pub fn load_u8(offs: u64) -> u8;
    pub fn store_u64(offs: u64, data: u64);
    pub fn load_u64(offs: u64) -> u64;
    pub fn config_get(offs: u64) -> u64;
    pub fn var_get(offs: u64) -> u64;
    pub fn var_set(offs: u64, offs1: u64);
    pub fn http_request(req: u64, body: u64) -> u64;
    pub fn http_status_code() -> i32;
    pub fn http_headers() -> u64;
    pub fn log_info(offs: u64);
    pub fn log_debug(offs: u64);
    pub fn log_warn(offs: u64);
    pub fn log_error(offs: u64);
    pub fn log_trace(offs: u64);
    pub fn get_log_level() -> i32;
}

/// Loads a byte array from Extism's memory. Only use this if you
/// have already considered the plugin_fn macro as well as the `extism_load_input` function.
///
/// # Arguments
///
/// * `offs` - The Extism offset pointer location to the memory
/// * `data` - The pointer to byte slice result
pub unsafe fn load(offs: u64, data: &mut [u8]) {
    let len = data.len();
    // x >> 3 == x / 8
    let chunk_count = len >> 3;

    let mut_ptr = data.as_mut_ptr() as *mut u64;
    for chunk_idx in 0..chunk_count {
        let x = load_u64(offs + (chunk_idx << 3) as u64);
        mut_ptr.add(chunk_idx).write(x);
    }

    // x % 8 == x & 7
    let remainder = len & 7;
    let remainder_offset = chunk_count << 3;
    // Allow the needless range loop because clippy wants to turn this into
    // iter_mut().enumerate().skip().take(), which is less readable IMO!
    #[allow(clippy::needless_range_loop)]
    for index in remainder_offset..(remainder + remainder_offset) {
        data[index] = load_u8(offs + index as u64);
    }
}

/// Loads the input from the host as a raw byte vec.
/// Consider using the plugin_fn macro to automatically
/// handle inputs as function parameters.
pub unsafe fn load_input() -> Vec<u8> {
    let len = input_length() as usize;
    let mut data = vec![0; len];
    let chunk_count = len >> 3;

    let mut_ptr = data.as_mut_ptr() as *mut u64;
    for chunk_idx in 0..chunk_count {
        let x = input_load_u64((chunk_idx << 3) as u64);
        mut_ptr.add(chunk_idx).write(x);
    }

    let remainder = len & 7;
    let remainder_offset = chunk_count << 3;
    #[allow(clippy::needless_range_loop)]
    for index in remainder_offset..(remainder + remainder_offset) {
        data[index] = input_load_u8(index as u64);
    }

    data
}

/// Stores a byte array into Extism's memory.
/// Only use this after considering []
///
/// # Arguments
///
/// * `offs` - The Extism offset pointer location to store the memory
/// * `data` - The byte array to store at that offset
pub unsafe fn store(offs: u64, data: &[u8]) {
    let len = data.len();
    let chunk_count = len >> 3;

    let ptr = data.as_ptr() as *const u64;
    for chunk_idx in 0..chunk_count {
        store_u64(offs + (chunk_idx << 3) as u64, ptr.add(chunk_idx).read());
    }

    let remainder = len & 7;
    let remainder_offset = chunk_count << 3;
    #[allow(clippy::needless_range_loop)]
    for index in remainder_offset..(remainder + remainder_offset) {
        store_u8(offs + index as u64, data[index]);
    }
}
