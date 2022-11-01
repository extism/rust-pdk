use crate::*;

pub struct Host;

impl Default for Host {
    fn default() -> Self {
        Host::new()
    }
}

pub struct HttpResponse {
    memory: Memory,
}

impl HttpResponse {
    pub fn into_memory(self) -> Memory {
        self.memory
    }

    pub fn as_memory(&self) -> &Memory {
        &self.memory
    }

    pub fn body(&self) -> Vec<u8> {
        self.memory.to_vec()
    }
}

impl Host {
    pub fn new() -> Host {
        Host
    }

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

    pub fn http_request(
        &self,
        req: &extism_manifest::HttpRequest,
        body: Option<&[u8]>,
    ) -> Result<HttpResponse, serde_json::Error> {
        let enc = serde_json::to_vec(req)?;
        let req = Memory::from_bytes(&enc);
        let body = body.map(Memory::from_bytes);
        let data = body.map(|x| x.offset).unwrap_or(0);
        let res = unsafe { extism_http_request(req.offset, data) };
        let len = unsafe { extism_length(res) };
        Ok(HttpResponse {
            memory: Memory::wrap(res, len),
        })
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

    pub fn config(&self, key: impl AsRef<str>) -> Option<String> {
        let mem = Memory::from_bytes(key.as_ref().as_bytes());

        let offset = unsafe { extism_config_get(mem.offset) };
        if offset == 0 {
            return None;
        }

        let len = unsafe { extism_length(offset) };
        if len == 0 {
            return None;
        }

        Some(
            Memory::wrap(offset, len)
                .to_string()
                .expect("Config value is not a valid string"),
        )
    }

    pub fn vars(&self) -> Vars {
        Vars::new(self)
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
