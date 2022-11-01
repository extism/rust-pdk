use crate::*;

#[derive(Clone, Copy)]
pub struct Host;

impl Host {
    pub fn input_bytes(&self) -> Vec<u8> {
        unsafe { extism_load_input() }
    }

    pub fn input_string(&self) -> Result<String, Error> {
        let s = String::from_utf8(self.input_bytes())?;
        Ok(s)
    }

    pub fn input<T: Input>(&self) -> Result<T, Error> {
        Input::input(self.input_bytes())
    }

    pub fn set_output_bytes(&self, data: impl AsRef<[u8]>) {
        let memory = Memory::from_bytes(data).keep();
        self.set_output_memory(&memory);
    }

    pub fn set_output_memory(&self, memory: &Memory) {
        unsafe {
            extism_output_set(memory.offset, memory.length);
        }
    }

    pub fn vars(&self) -> Vars {
        Vars
    }

    pub fn config(&self) -> Config {
        Config
    }

    pub fn log_memory(&self, level: LogLevel, memory: &Memory) {
        unsafe {
            match level {
                LogLevel::Info => extism_log_info(memory.offset),
                LogLevel::Debug => extism_log_debug(memory.offset),
                LogLevel::Warn => extism_log_warn(memory.offset),
                LogLevel::Error => extism_log_error(memory.offset),
            }
        }
    }

    pub fn log(&self, level: LogLevel, str: &str) {
        log!(level, "{}", str)
    }
}
