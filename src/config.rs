use crate::*;

pub fn get(key: impl AsRef<str>) -> Option<String> {
    let mem = Memory::from_bytes(key.as_ref().as_bytes());

    let offset = unsafe { extism_config_get(mem.offset) };
    if offset == 0 {
        return None;
    }

    let len = unsafe { extism_length(offset) };
    if len == 0 {
        return None;
    }

    Some(
        Memory::wrap(offset, len, true)
            .to_string()
            .expect("Config value is not a valid string"),
    )
}
