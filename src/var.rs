use crate::*;

pub fn get_memory(key: impl AsRef<str>) -> Option<Memory> {
    let mem = Memory::from_bytes(key.as_ref().as_bytes());

    let offset = unsafe { extism_var_get(mem.offset) };
    if offset == 0 {
        return None;
    }
    let length = unsafe { extism_length(offset) };

    if length == 0 {
        return None;
    }

    let memory = Memory::wrap(offset, length);
    Some(memory)
}

pub fn get(key: impl AsRef<str>) -> Option<Vec<u8>> {
    get_memory(key).map(|x| x.to_vec())
}

pub fn set(key: impl AsRef<str>, val: impl AsRef<[u8]>) {
    let key = Memory::from_bytes(key.as_ref().as_bytes());
    let val = Memory::from_bytes(val.as_ref());
    unsafe { extism_var_set(key.offset, val.offset) }
}

pub fn set_memory(key: impl AsRef<str>, val: &Memory) {
    let key = Memory::from_bytes(key.as_ref().as_bytes());
    unsafe { extism_var_set(key.offset, val.offset) }
}

pub fn remove(key: impl AsRef<str>) {
    let key = Memory::from_bytes(key.as_ref().as_bytes());
    unsafe { extism_var_set(key.offset, 0) }
}
