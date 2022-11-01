use crate::*;

pub trait Output {
    fn output(&self) -> Result<Memory, Error>;
}

impl Output for Memory {
    fn output(&self) -> Result<Memory, Error> {
        Ok(Memory::wrap(self.offset, self.length).keep())
    }
}

impl Output for HttpResponse {
    fn output(&self) -> Result<Memory, Error> {
        self.as_memory().output()
    }
}

impl Output for () {
    fn output(&self) -> Result<Memory, Error> {
        Ok(Memory::null())
    }
}

impl Output for Vec<u8> {
    fn output(&self) -> Result<Memory, Error> {
        Ok(Memory::from_bytes(self))
    }
}

impl Output for &[u8] {
    fn output(&self) -> Result<Memory, Error> {
        Ok(Memory::from_bytes(self))
    }
}

impl Output for String {
    fn output(&self) -> Result<Memory, Error> {
        Ok(Memory::from_bytes(self))
    }
}

impl Output for &str {
    fn output(&self) -> Result<Memory, Error> {
        Ok(Memory::from_bytes(self))
    }
}

impl Output for json::Value {
    fn output(&self) -> Result<Memory, Error> {
        Ok(Memory::from_bytes(serde_json::to_vec(self)?))
    }
}
