use crate::*;

pub fn input_bytes() -> Vec<u8> {
    unsafe { extism_load_input() }
}

pub fn input_string() -> Result<String, Error> {
    let s = String::from_utf8(input_bytes())?;
    Ok(s)
}

pub fn input<T: Input>() -> Result<T, Error> {
    Input::input(input_bytes())
}

pub fn set_output_bytes(data: impl AsRef<[u8]>) {
    let memory = Memory::from_bytes(data).keep();
    set_output_memory(&memory);
}

pub fn set_output_memory(memory: &Memory) {
    unsafe {
        extism_output_set(memory.offset, memory.length);
    }
}

pub fn log_memory(level: LogLevel, memory: &Memory) {
    unsafe {
        match level {
            LogLevel::Info => extism_log_info(memory.offset),
            LogLevel::Debug => extism_log_debug(memory.offset),
            LogLevel::Warn => extism_log_warn(memory.offset),
            LogLevel::Error => extism_log_error(memory.offset),
        }
    }
}
