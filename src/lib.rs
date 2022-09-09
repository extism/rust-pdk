pub mod bindings;
mod pointer;

use bindings::*;
pub use pointer::Pointer;

pub struct Host {
    input: Vec<u8>,
}

impl Default for Host {
    fn default() -> Self {
        Host::new()
    }
}

pub struct Vars<'a>(&'a Host);

pub struct Memory {
    should_free: bool,
    offset: u64,
    length: u64,
}

impl Memory {
    pub fn new(length: usize, should_free: bool) -> Memory {
        let length = length as u64;
        let offset = unsafe { extism_alloc(length) };
        Memory {
            offset,
            length,
            should_free,
        }
    }

    pub fn wrap(offset: u64, length: u64) -> Memory {
        Memory {
            length,
            offset,
            should_free: false,
        }
    }

    pub fn load(&self, mut buf: impl AsMut<[u8]>) {
        let buf = buf.as_mut();
        unsafe { extism_load(self.offset, &mut buf[0..self.length as usize]) }
    }

    pub fn store(&mut self, buf: impl AsRef<[u8]>) {
        let buf = buf.as_ref();
        unsafe { extism_store(self.offset, &buf[0..self.length as usize]) }
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        if self.should_free {
            unsafe { extism_free(self.offset) }
        }
    }
}

impl<'a> Vars<'a> {
    pub fn new(host: &'a Host) -> Self {
        Vars(host)
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<Pointer<Vec<u8>>> {
        let mem = self.0.alloc_bytes(key.as_ref().as_bytes());

        let offset = unsafe { extism_var_get(mem.offset) };
        if offset == 0 {
            return None;
        }
        let len = unsafe { extism_length(offset) };

        if len == 0 {
            return None;
        }

        let mut buf = vec![0; len as usize];
        unsafe {
            extism_load(offset, &mut buf);
        }
        Some(Pointer::new(buf, Memory::wrap(len, offset)))
    }

    pub fn set(&mut self, key: impl AsRef<str>, val: impl AsRef<[u8]>) {
        let key = self.0.alloc_bytes(key.as_ref().as_bytes());
        let val = self.0.alloc_bytes(val.as_ref());
        unsafe { extism_var_set(key.offset, val.offset) }
    }

    pub fn set_memory(&mut self, key: impl AsRef<str>, val: &Memory) {
        let key = self.0.alloc_bytes(key.as_ref().as_bytes());
        unsafe { extism_var_set(key.offset, val.offset) }
    }

    pub fn remove(&mut self, key: impl AsRef<str>) {
        let key = self.0.alloc_bytes(key.as_ref().as_bytes());
        unsafe { extism_var_set(key.offset, 0) }
    }
}

impl Host {
    pub fn new() -> Host {
        unsafe {
            let input_offset = extism_input_offset();
            let input_length = extism_length(input_offset);
            let mut input = vec![0; input_length as usize];
            extism_load(input_offset, &mut input);
            Host { input }
        }
    }

    pub fn alloc(&self, length: usize) -> Memory {
        Memory::new(length, true)
    }

    pub fn alloc_bytes(&self, data: impl AsRef<[u8]>) -> Memory {
        let data = data.as_ref();
        let length = data.len();
        let mut memory = Memory::new(length, true);
        memory.store(data);
        memory
    }

    pub fn input(&self) -> &[u8] {
        self.input.as_slice()
    }

    pub fn input_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.input.as_slice()) }
    }

    pub fn http_request(
        &self,
        req: &extism_manifest::HttpRequest,
        body: Option<&[u8]>,
    ) -> Result<Pointer<Vec<u8>>, serde_json::Error> {
        let enc = serde_json::to_vec(req)?;
        let req = self.alloc_bytes(&enc);
        let body = match body {
            Some(b) => self.alloc_bytes(b).offset,
            None => 0,
        };
        let res = unsafe { extism_http_request(req.offset, body) };
        let len = unsafe { extism_length(res) };
        let mut dest = vec![0; len as usize];
        unsafe { bindings::extism_load(res, &mut dest) };
        Ok(Pointer::new(dest, Memory::wrap(res, len)))
    }

    pub fn output(&self, data: impl AsRef<[u8]>) {
        let len = data.as_ref().len();
        unsafe {
            let offs = extism_alloc(len as u64);
            extism_store(offs, data.as_ref());
            extism_output_set(offs, len as u64);
        }
    }

    pub fn output_memory(&self, memory: &Memory) {
        unsafe {
            extism_output_set(memory.offset, memory.length);
        }
    }

    pub fn config(&self, key: impl AsRef<str>) -> Option<Pointer<String>> {
        let mem = self.alloc_bytes(key.as_ref().as_bytes());

        let offset = unsafe { extism_config_get(mem.offset) };
        if offset == 0 {
            return None;
        }

        let len = unsafe { extism_length(offset) };

        if len == 0 {
            return None;
        }

        let mut buf = vec![0; len as usize];
        unsafe {
            extism_load(offset, &mut buf);
            Some(Pointer::new(
                String::from_utf8_unchecked(buf),
                Memory::wrap(offset, len),
            ))
        }
    }

    pub fn vars(&self) -> Vars {
        Vars::new(self)
    }
}
