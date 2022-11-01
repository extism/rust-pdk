use crate::*;

pub struct Memory {
    pub should_free: bool,
    pub offset: u64,
    pub length: u64,
}

impl Memory {
    pub fn null() -> Memory {
        Memory::wrap(0, 0)
    }

    pub fn new(length: usize) -> Memory {
        let length = length as u64;
        let offset = unsafe { extism_alloc(length) };
        Memory {
            offset,
            length,
            should_free: true,
        }
    }

    pub(crate) fn wrap(offset: u64, length: u64) -> Memory {
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

    pub fn keep(mut self) -> Self {
        self.should_free = false;
        self
    }

    pub fn from_bytes(data: impl AsRef<[u8]>) -> Memory {
        let data = data.as_ref();
        let length = data.len();
        let mut memory = Memory::new(length);
        memory.store(data);
        memory
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut dest = vec![0u8; self.length as usize];
        self.load(&mut dest);
        dest
    }

    pub fn to_string(&self) -> Result<String, Error> {
        let x = String::from_utf8(self.to_vec())?;
        Ok(x)
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        if self.should_free {
            unsafe { extism_free(self.offset) }
        }
    }
}
