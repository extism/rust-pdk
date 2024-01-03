use crate::*;

pub struct Memory(pub MemoryHandle);

pub mod internal {
    use super::*;

    pub fn memory_alloc(n: u64) -> MemoryHandle {
        let length = n;
        let offset = unsafe { extism::alloc(length) };
        MemoryHandle { offset, length }
    }

    pub fn memory_free(handle: MemoryHandle) {
        unsafe { extism::free(handle.offset) }
    }

    pub fn memory_bytes(handle: MemoryHandle) -> Vec<u8> {
        let mut data = vec![0; handle.offset as usize];
        unsafe { extism::load(handle.offset, &mut data) };
        data
    }

    /// Get the length of the memory handle stored at the given offset, this will return 0 if called on a non-handle pointer
    pub fn memory_length(offs: u64) -> u64 {
        unsafe { extism::length(offs) }
    }

    /// Get the length of the memory handle stored at the given offset, this may return garbage if called on a non-handle pointer
    pub fn memory_length_unsafe(offs: u64) -> u64 {
        unsafe { extism::length_unsafe(offs) }
    }

    /// Load data from memory into a `u8` slice
    pub fn load(handle: MemoryHandle, mut buf: impl AsMut<[u8]>) {
        let buf = buf.as_mut();
        unsafe {
            extism::load(handle.offset, &mut buf[0..handle.length as usize]);
        }
    }

    /// Load data from `u8` slice into memory
    pub fn store(handle: MemoryHandle, buf: impl AsRef<[u8]>) {
        let buf = buf.as_ref();
        unsafe { extism::store(handle.offset, &buf[0..handle.length as usize]) }
    }

    /// Find `Memory` by offset
    pub fn find(offset: u64) -> Option<MemoryHandle> {
        let length = unsafe { extism::length(offset) };

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
        let data = x.to_bytes()?;
        let data = data.as_ref();
        let length = data.len() as u64;
        let offset = unsafe { extism::alloc(length) };
        unsafe { extism::store(offset, data) };
        Ok(Self(MemoryHandle { offset, length }))
    }

    /// Create a memory block and copy bytes from `u8` slice
    pub fn from_bytes(data: impl AsRef<[u8]>) -> Result<Self, Error> {
        let memory = Memory::new(&data.as_ref())?;
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
    pub fn set_output(self) {
        unsafe {
            extism::output_set(self.0.offset, self.0.length);
        }
    }

    /// Log memory
    pub fn log(&self, level: LogLevel) {
        unsafe {
            match level {
                LogLevel::Info => extism::log_info(self.0.offset),
                LogLevel::Debug => extism::log_debug(self.0.offset),
                LogLevel::Warn => extism::log_warn(self.0.offset),
                LogLevel::Error => extism::log_error(self.0.offset),
            }
        }
    }

    /// Convert to a Rust value
    pub fn to<T: FromBytesOwned>(&self) -> Result<T, Error> {
        T::from_bytes_owned(&self.to_vec())
    }

    /// Locate a memory block by offset
    pub fn find(offs: u64) -> Option<Memory> {
        internal::find(offs).map(Memory)
    }

    /// Free a memory block, allowing for it to be re-used
    pub fn free(self) {
        internal::memory_free(self.0)
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
