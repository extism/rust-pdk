use crate::*;

pub fn get_memory(key: impl AsRef<str>) -> Result<Option<Memory>, Error> {
    let mem = Memory::from_bytes(key.as_ref().as_bytes())?;

    let offset = unsafe { extism::config_get(mem.offset()) };
    if offset == 0 {
        return Ok(None);
    }

    let len = unsafe { extism::length(offset) };
    if len == 0 {
        return Ok(None);
    }

    Ok(Some(Memory(MemoryHandle {
        offset,
        length: len,
    })))
}

/// Gets a config item passed in from the host. This item is read-only
/// and static throughout the lifetime of the plug-in.
///
/// # Arguments
///
/// * `key` - A unique string key to identify the variable
///
/// # Examples
///
/// ```
/// // let's assume we have a config object: { my_config: 42u32 }
/// // which is a u32. We can default to 0 first time we fetch it if it's not present
/// let my_config = config::get("my_config")?.unwrap_or(0u32);
/// ```
pub fn get(key: impl AsRef<str>) -> Result<Option<String>, Error> {
    Ok(get_memory(key)?.map(|x| x.to_string().expect("Config value is not a valid string")))
}
