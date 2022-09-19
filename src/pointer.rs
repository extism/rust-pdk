use crate::*;

// The `Pointer` type is used to create values from the private plugin memory that
// are associated with values on the heap/stack
pub struct Pointer<T> {
    value: T,
    memory: Memory,
    encode_bytes: Box<dyn Fn(&T) -> &[u8]>,
}

impl<T> Pointer<T> {
    /// Create a new pointer from the given value and memory region, the memory length should
    /// have the same size as `T`
    ///
    /// The `encode_bytes` parameter is used to provide a function that can convert `T` back to bytes
    pub fn new(
        value: T,
        memory: Memory,
        encode_bytes: impl 'static + Fn(&T) -> &[u8],
    ) -> Pointer<T> {
        assert!(std::mem::size_of::<T>() as u64 <= memory.length);
        Pointer {
            value,
            memory,
            encode_bytes: Box::new(encode_bytes),
        }
    }

    /// Allocate a new pointer for the given type, if `free` is `true` then the memory block will
    /// be freed when this value goes out of scope.
    pub fn make(free: bool) -> Pointer<T> {
        let memory = Memory::new(std::mem::size_of::<T>(), free);
        let mut x = std::mem::MaybeUninit::zeroed();
        let ptr = x.as_mut_ptr();
        unsafe {
            let slice = std::slice::from_raw_parts_mut(ptr as *mut _, memory.length as usize);
            extism_load(memory.offset, slice);
            Pointer::new(x.assume_init(), memory, |x| {
                std::slice::from_raw_parts(x as *const _ as *const u8, std::mem::size_of::<T>())
            })
        }
    }

    /// Get the inner value
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Get the memory offset
    pub fn offset(&self) -> u64 {
        self.memory.offset
    }

    /// Get the memory handle
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    /// Don't free when dropped
    pub fn keep(mut self) -> Self {
        self.memory = self.memory.keep();
        self
    }

    /// Save value back to the plugin memory
    pub fn save(&mut self) {
        self.memory.store((self.encode_bytes)(&self.value))
    }
}

impl Pointer<String> {
    /// Create a pointer to a string
    pub fn string(memory: Memory) -> Pointer<String> {
        let mut buf: Vec<u8> = vec![0; memory.length as usize];
        unsafe { extism_load(memory.offset, &mut buf) };
        let str = unsafe { String::from_utf8_unchecked(buf) };
        Pointer::new(str, memory, |x| x.as_bytes())
    }
}

impl<T: Default + Clone> Pointer<Vec<T>> {
    /// Create a pointer to a `Vec`
    pub fn vec(memory: Memory) -> Pointer<Vec<T>> {
        let mut buf = vec![Default::default(); memory.length as usize / std::mem::size_of::<T>()];
        unsafe {
            let mut slice =
                std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut _, memory.length as usize);
            extism_load(memory.offset, &mut slice);
        }
        Pointer::new(buf, memory, |x| unsafe {
            std::slice::from_raw_parts(x.as_ptr() as *const u8, x.len() * std::mem::size_of::<T>())
        })
    }
}

impl<T> AsRef<T> for Pointer<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for Pointer<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl AsRef<[u8]> for Pointer<Vec<u8>> {
    fn as_ref(&self) -> &[u8] {
        &self.value
    }
}

impl AsMut<[u8]> for Pointer<Vec<u8>> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.value
    }
}

impl From<Memory> for Pointer<String> {
    fn from(mem: Memory) -> Self {
        Pointer::string(mem)
    }
}

impl<T: Default + Clone> From<Memory> for Pointer<Vec<T>> {
    fn from(mem: Memory) -> Self {
        Pointer::vec(mem)
    }
}
