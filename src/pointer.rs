use crate::*;

pub struct Pointer<T> {
    value: T,
    memory: Memory,
}

impl<T> Pointer<T> {
    pub fn new(value: T, memory: Memory) -> Pointer<T> {
        Pointer { value, memory }
    }

    pub fn make(free: bool) -> Pointer<T> {
        let memory = Memory::new(std::mem::size_of::<T>(), free);
        let mut x = std::mem::MaybeUninit::zeroed();
        let ptr = x.as_mut_ptr();
        unsafe {
            let slice = std::slice::from_raw_parts_mut(ptr as *mut _, memory.length as usize);
            extism_load(memory.offset, slice);
            Pointer::new(x.assume_init(), memory)
        }
    }

    pub fn into_inner(self) -> T {
        self.value
    }

    pub fn offset(&self) -> u64 {
        self.memory.offset
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }
}

impl Pointer<String> {
    pub fn string(memory: Memory) -> Pointer<String> {
        let mut buf: Vec<u8> = vec![0; memory.length as usize];
        unsafe { extism_load(memory.offset, &mut buf) };
        let str = unsafe { String::from_utf8_unchecked(buf) };
        Pointer::new(str, memory)
    }
}

impl<T: Default + Clone> Pointer<Vec<T>> {
    pub fn vec(memory: Memory) -> Pointer<Vec<T>> {
        let mut buf = vec![Default::default(); memory.length as usize / std::mem::size_of::<T>()];
        unsafe {
            let mut slice =
                std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut _, memory.length as usize);
            extism_load(memory.offset, &mut slice);
        }
        Pointer::new(buf, memory)
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
