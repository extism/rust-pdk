#[link(wasm_import_module = "extism:host/env")]
extern "C" {
    pub fn input_length() -> u64;
    pub fn input_load_u8(offs: u64) -> u8;
    pub fn input_load_u64(offs: u64) -> u64;
    pub fn length(offs: u64) -> u64;
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
    pub fn log_info(offs: u64);
    pub fn log_debug(offs: u64);
    pub fn log_warn(offs: u64);
    pub fn log_error(offs: u64);
}

/// Loads a byte array from Extism's memory. Only use this if you
/// have already considered the plugin_fn macro as well as the [extism_load_input] function.
///
/// # Arguments
///
/// * `offs` - The Extism offset pointer location to the memory
/// * `data` - The pointer to byte slice result
pub unsafe fn load(offs: u64, data: &mut [u8]) {
    let ptr = data.as_mut_ptr();

    let mut index = 0;
    let mut left;
    let len = data.len();
    while index < len {
        left = len - index;
        if left < 8 {
            data[index] = load_u8(offs + index as u64);
            index += 1;
            continue;
        }

        let x = load_u64(offs + index as u64);
        (ptr as *mut u64).add(index / 8).write(x);
        index += 8;
    }
}

/// Loads the input from the host as a raw byte vec.
/// Consider using the plugin_fn macro to automatically
/// handle inputs as function parameters.
pub unsafe fn load_input() -> Vec<u8> {
    let input_length = input_length();
    let mut data = vec![0; input_length as usize];

    let mut index = 0;
    let mut left;
    let len = data.len();
    while index < len {
        left = len - index;
        if left < 8 {
            data[index] = input_load_u8(index as u64);
            index += 1;
            continue;
        }

        let x = input_load_u64(index as u64);
        (data.as_mut_ptr() as *mut u64).add(index / 8).write(x);
        index += 8;
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
    let ptr = data.as_ptr();

    let mut index = 0;
    let mut left;
    let len = data.len();
    while index < len {
        left = len - index;
        if left < 8 {
            store_u8(offs + index as u64, data[index]);
            index += 1;
            continue;
        }

        store_u64(
            offs + index as u64,
            (ptr as *const u64).add(index / 8).read(),
        );
        index += 8;
    }
}
