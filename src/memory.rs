use crate::*;

pub struct Memory(pub MemoryHandle);

pub mod internal {
    use super::*;

    fn memory_alloc(n: u64) -> MemoryHandle {
        let length = n as u64;
        let offset = unsafe { extism_alloc(length) };
        MemoryHandle { offset, length }
    }

    fn memory_free(handle: MemoryHandle) {
        unsafe { extism_free(handle.offset) }
    }

    fn memory_bytes(handle: MemoryHandle) -> Vec<u8> {
        let mut data = vec![0; handle.offset as usize];
        unsafe { extism_load(handle.offset, &mut data) };
        data
    }

    fn memory_length(offs: u64) -> u64 {
        unsafe { extism_length(offs) }
    }
    /// Load data from memory into a `u8` slice
    pub fn load(handle: MemoryHandle, mut buf: impl AsMut<[u8]>) {
        let buf = buf.as_mut();
        unsafe {
            extism_load(handle.offset, &mut buf[0..handle.length as usize]);
        }
    }

    /// Load data from `u8` slice into memory
    pub fn store(handle: MemoryHandle, buf: impl AsRef<[u8]>) {
        let buf = buf.as_ref();
        unsafe { extism_store(handle.offset, &buf[0..handle.length as usize]) }
    }

    /// Find `Memory` by offset
    pub fn find(offset: u64) -> Option<MemoryHandle> {
        let length = unsafe { bindings::extism_length(offset) };

        if length == 0 {
            return None;
        }

        Some(MemoryHandle { offset, length })
    }
}

impl Memory {
    pub fn offset(&self) -> u64 {
        self.0.offset
    }

    pub fn len(&self) -> usize {
        self.0.length as usize
    }

    pub fn is_empty(&self) -> bool {
        self.0.length == 0
    }

    pub fn null() -> Self {
        Memory(MemoryHandle {
            offset: 0,
            length: 0,
        })
    }

    /// Allocate a new block with an encoded value
    pub fn new<'a, T: ToBytes<'a>>(x: &T) -> Result<Self, Error> {
        let data = x.to_bytes()?.as_ref();
        let length = data.len() as u64;
        let offset = unsafe { extism_alloc(length) };
        unsafe { extism_store(offset, &data) };
        Ok(Self(MemoryHandle { offset, length }))
    }

    /// Create a memory block and copy bytes from `u8` slice
    pub fn from_bytes(data: impl AsRef<[u8]>) -> Result<Self, Error> {
        let mut memory = Memory::new(&data.as_ref())?;
        Ok(memory)
    }

    /// Copy data out of memory and into a vec
    pub fn to_vec(&self) -> Vec<u8> {
        let mut dest = vec![0u8; self.0.length as usize];
        internal::load(self.0, &mut dest);
        dest
    }

    /// Copy data out of memory and convert to string
    pub fn to_string(&self) -> Result<String, Error> {
        let x = String::from_utf8(self.to_vec())?;
        Ok(x)
    }

    /// Store memory as function output
    pub fn set_output(mut self) {
        unsafe {
            extism_output_set(self.0.offset, self.0.length);
        }
    }

    /// Log memory
    pub fn log(&self, level: LogLevel) {
        unsafe {
            match level {
                LogLevel::Info => extism_log_info(self.0.offset),
                LogLevel::Debug => extism_log_debug(self.0.offset),
                LogLevel::Warn => extism_log_warn(self.0.offset),
                LogLevel::Error => extism_log_error(self.0.offset),
            }
        }
    }

    pub fn to<'a, T: FromBytes<'a>>(&self) -> Result<T, Error> {
        T::from_bytes(&self.to_vec())
    }

    pub fn find(offs: u64) -> Option<Memory> {
        internal::find(offs).map(Memory)
    }
}

impl From<Memory> for () {
    fn from(_: Memory) {}
}

impl From<()> for Memory {
    fn from(_: ()) -> Memory {
        Memory(MemoryHandle::null())
    }
}

impl From<Memory> for i64 {
    fn from(m: Memory) -> Self {
        m.0.offset as i64
    }
}

impl From<Memory> for u64 {
    fn from(m: Memory) -> Self {
        m.0.offset
    }
}

impl From<u64> for Memory {
    fn from(offset: u64) -> Memory {
        Memory::find(offset).unwrap_or_else(Memory::null)
    }
}

impl From<i64> for Memory {
    fn from(offset: i64) -> Memory {
        Memory::find(offset as u64).unwrap_or_else(Memory::null)
    }
}
