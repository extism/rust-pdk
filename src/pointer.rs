use crate::*;

pub struct Pointer<T> {
    value: T,
    memory: Memory,
}

impl<T> Pointer<T> {
    pub fn new(value: T, memory: Memory) -> Pointer<T> {
        Pointer { value, memory }
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
