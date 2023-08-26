use crate::*;

pub fn get_memory(key: impl AsRef<str>) -> Result<Option<Memory>, Error> {
    let mem = Memory::from_bytes(key.as_ref().as_bytes())?;

    let offset = unsafe { extism_config_get(mem.offset()) };
    if offset == 0 {
        return Ok(None);
    }

    let len = unsafe { extism_length(offset) };
    if len == 0 {
        return Ok(None);
    }

    Ok(Some(Memory(MemoryHandle {
        offset,
        length: len,
    })))
}

pub fn get(key: impl AsRef<str>) -> Result<Option<String>, Error> {
    Ok(get_memory(key)?.map(|x| x.to_string().expect("Config value is not a valid string")))
}
