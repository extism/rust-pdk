use crate::*;

/// Memory is used to track access to host memory blocks
pub struct Memory {
    /// When `true` the block will be freed when it goes out of scope
    pub free: bool,
    /// Memory offset
    pub offset: u64,
    /// Memory length
    pub length: u64,
}

impl Memory {
    /// `NULL` equivalent
    pub fn null() -> Memory {
        Memory::wrap(0, 0, false)
    }

    pub fn len(&self) -> usize {
        self.length as usize
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Allocate a new block with the given `size`
    pub fn new(length: usize) -> Memory {
        let length = length as u64;
        let offset = unsafe { extism_alloc(length) };
        Memory {
            offset,
            length,
            free: true,
        }
    }

    pub(crate) fn wrap(offset: u64, length: u64, free: bool) -> Memory {
        Memory {
            length,
            offset,
            free,
        }
    }

    /// Load data from memory into a `u8` slice
    pub fn load(&self, mut buf: impl AsMut<[u8]>) {
        let buf = buf.as_mut();
        unsafe { extism_load(self.offset, &mut buf[0..self.length as usize]) }
    }

    /// Load data from `u8` slice into memory
    pub fn store(&mut self, buf: impl AsRef<[u8]>) {
        let buf = buf.as_ref();
        unsafe { extism_store(self.offset, &buf[0..self.length as usize]) }
    }

    /// Prevent memory from being freed when it goes out of scope.     
    pub fn keep(mut self) -> Self {
        self.free = false;
        self
    }

    /// Create a memory block and copy bytes from `u8` slice
    pub fn from_bytes(data: impl AsRef<[u8]>) -> Memory {
        let data = data.as_ref();
        let length = data.len();
        let mut memory = Memory::new(length);
        memory.store(data);
        memory
    }

    /// Copy data out of memory and into a vec
    pub fn to_vec(&self) -> Vec<u8> {
        let mut dest = vec![0u8; self.length as usize];
        self.load(&mut dest);
        dest
    }

    /// Copy data out of memory and convert to string
    pub fn to_string(&self) -> Result<String, Error> {
        let x = String::from_utf8(self.to_vec())?;
        Ok(x)
    }

    /// Store memory as function output
    pub fn set_output(mut self) {
        self = self.keep();
        unsafe {
            extism_output_set(self.offset, self.length);
        }
    }

    /// Log memory
    pub fn log(&self, level: LogLevel) {
        unsafe {
            match level {
                LogLevel::Info => extism_log_info(self.offset),
                LogLevel::Debug => extism_log_debug(self.offset),
                LogLevel::Warn => extism_log_warn(self.offset),
                LogLevel::Error => extism_log_error(self.offset),
            }
        }
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        if self.free {
            unsafe { extism_free(self.offset) }
        }
    }
}
