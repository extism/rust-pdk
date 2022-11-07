use crate::*;

pub fn get_memory(key: impl AsRef<str>) -> Option<Memory> {
    let mem = Memory::from_bytes(key.as_ref().as_bytes());

    let offset = unsafe { extism_config_get(mem.offset) };
    if offset == 0 {
        return None;
    }

    let len = unsafe { extism_length(offset) };
    if len == 0 {
        return None;
    }

    Some(Memory::wrap(offset, len, true))
}

pub fn get(key: impl AsRef<str>) -> Option<String> {
    get_memory(key).map(|x| x.to_string().expect("Config value is not a valid string"))
}
