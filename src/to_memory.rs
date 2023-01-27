use crate::*;

use base64::Engine;

pub trait ToMemory {
    fn to_memory(&self) -> Result<Memory, Error>;

    fn status(&self) -> i32 {
        0
    }
}

impl ToMemory for Memory {
    fn to_memory(&self) -> Result<Memory, Error> {
        Ok(Memory::wrap(self.offset, self.length, false))
    }
}

#[cfg(feature = "http")]
impl ToMemory for HttpResponse {
    fn to_memory(&self) -> Result<Memory, Error> {
        self.as_memory().to_memory()
    }
}

impl ToMemory for () {
    fn to_memory(&self) -> Result<Memory, Error> {
        Ok(Memory::null())
    }
}

impl ToMemory for Vec<u8> {
    fn to_memory(&self) -> Result<Memory, Error> {
        Ok(Memory::from_bytes(self))
    }
}

impl ToMemory for &[u8] {
    fn to_memory(&self) -> Result<Memory, Error> {
        Ok(Memory::from_bytes(self))
    }
}

impl ToMemory for String {
    fn to_memory(&self) -> Result<Memory, Error> {
        Ok(Memory::from_bytes(self))
    }
}

impl ToMemory for &str {
    fn to_memory(&self) -> Result<Memory, Error> {
        Ok(Memory::from_bytes(self))
    }
}

impl ToMemory for json::Value {
    fn to_memory(&self) -> Result<Memory, Error> {
        Ok(Memory::from_bytes(serde_json::to_vec(self)?))
    }
}

impl ToMemory for Base64 {
    fn to_memory(&self) -> Result<Memory, Error> {
        base64::engine::general_purpose::STANDARD
            .encode(&self.0)
            .to_memory()
    }
}

impl ToMemory for u64 {
    fn to_memory(&self) -> Result<Memory, Error> {
        let length = unsafe { bindings::extism_length(*self) };
        Ok(Memory {
            length,
            offset: *self,
            free: false,
        })
    }
}
