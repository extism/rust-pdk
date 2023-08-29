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
        Ok(Memory(MemoryHandle {
            offset: self.offset(),
            length: self.len() as u64,
        }))
    }
}

impl<'a> ToMemory for &'a Memory {
    fn to_memory(&self) -> Result<Memory, Error> {
        Ok(Memory(MemoryHandle {
            offset: self.offset(),
            length: self.len() as u64,
        }))
    }
}

#[cfg(feature = "http")]
impl ToMemory for HttpResponse {
    fn to_memory(&self) -> Result<Memory, Error> {
        self.as_memory().to_memory()
    }
}

impl<'a, T: ToBytes<'a>> ToMemory for T {
    fn to_memory(&self) -> Result<Memory, Error> {
        Memory::from_bytes(self.to_bytes()?)
    }
}

impl ToMemory for Base64 {
    fn to_memory(&self) -> Result<Memory, Error> {
        base64::engine::general_purpose::STANDARD
            .encode(&self.0)
            .as_str()
            .to_memory()
    }
}
