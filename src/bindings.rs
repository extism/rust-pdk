#[link(wasm_import_module = "extism:env")]
extern "C" {
    pub fn extism_input_length() -> u64;
    pub fn extism_input_load_u8(offs: u64) -> u8;
    pub fn extism_input_load_u64(offs: u64) -> u64;
    pub fn extism_length(offs: u64) -> u64;
    pub fn extism_alloc(length: u64) -> u64;
    pub fn extism_free(offs: u64);
    pub fn extism_output_set(offs: u64, length: u64);
    pub fn extism_error_set(offs: u64);
    pub fn extism_store_u8(offs: u64, data: u8);
    pub fn extism_load_u8(offs: u64) -> u8;
    pub fn extism_store_u64(offs: u64, data: u64);
    pub fn extism_load_u64(offs: u64) -> u64;
    pub fn extism_config_get(offs: u64) -> u64;
    pub fn extism_var_get(offs: u64) -> u64;
    pub fn extism_var_set(offs: u64, offs1: u64);
    pub fn extism_http_request(req: u64, body: u64) -> u64;
    pub fn extism_http_status_code() -> i32;
    pub fn extism_log_info(offs: u64);
    pub fn extism_log_debug(offs: u64);
    pub fn extism_log_warn(offs: u64);
    pub fn extism_log_error(offs: u64);
}

/// Loads a byte array from Extism's memory. Only use this if you
/// have already considered the plugin_fn macro as well as the [extism_load_input] function.
///
/// # Arguments
///
/// * `offs` - The Extism offset pointer location to the memory
/// * `data` - The pointer to byte slice result
pub unsafe fn extism_load(offs: u64, data: &mut [u8]) {
    let ptr = data.as_mut_ptr();

    let mut index = 0;
    let mut left;
    let len = data.len();
    while index < len {
        left = len - index;
        if left < 8 {
            data[index] = extism_load_u8(offs + index as u64);
            index += 1;
            continue;
        }

        let x = extism_load_u64(offs + index as u64);
        (ptr as *mut u64).add(index / 8).write(x);
        index += 8;
    }
}

/// Loads the input from the host as a raw byte vec.
/// Consider using the plugin_fn macro to automatically
/// handle inputs as function parameters.
pub unsafe fn extism_load_input() -> Vec<u8> {
    let input_length = extism_input_length();
    let mut data = vec![0; input_length as usize];

    let mut index = 0;
    let mut left;
    let len = data.len();
    while index < len {
        left = len - index;
        if left < 8 {
            data[index] = extism_input_load_u8(index as u64);
            index += 1;
            continue;
        }

        let x = extism_input_load_u64(index as u64);
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
pub unsafe fn extism_store(offs: u64, data: &[u8]) {
    let ptr = data.as_ptr();

    let mut index = 0;
    let mut left;
    let len = data.len();
    while index < len {
        left = len - index;
        if left < 8 {
            extism_store_u8(offs + index as u64, data[index]);
            index += 1;
            continue;
        }

        extism_store_u64(
            offs + index as u64,
            (ptr as *const u64).add(index / 8).read(),
        );
        index += 8;
    }
}
