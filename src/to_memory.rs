use crate::*;

pub trait ToMemory {
    fn to_memory(&self) -> Result<Memory, Error>;
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
