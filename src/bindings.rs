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

/// # Safety
///
/// This function is used to access WASM memory
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

/// # Safety
///
/// This function is used to access WASM memory
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

/// # Safety
///
/// This function is used to access WASM memory
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
