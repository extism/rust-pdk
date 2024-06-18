use crate::*;

pub fn get_memory(key: impl AsRef<str>) -> Result<Option<Memory>, Error> {
    let mem = Memory::from_bytes(key.as_ref().as_bytes())?;

    let offset = unsafe { extism::var_get(mem.offset()) };
    if offset == 0 {
        return Ok(None);
    }
    let length = unsafe { extism::length(offset) };

    if length == 0 {
        return Ok(None);
    }

    let memory = MemoryHandle { offset, length };
    Ok(Some(Memory(memory)))
}

/// Gets a variable in the plug-in. This variable lives as long as the
/// plug-in is loaded.
///
/// # Arguments
///
/// * `key` - A unique string key to identify the variable
///
/// # Examples
///
/// ```
/// // let's assume we have a variable at `"my_var"`
/// // which is a u32. We can default to 0 first time we fetch it:
/// let my_var = var::get("my_var")?.unwrap_or(0u32);
/// ```
pub fn get<T: FromBytesOwned>(key: impl AsRef<str>) -> Result<Option<T>, Error> {
    match get_memory(key)?.map(|x| {
        let res = x.to_vec();
        x.free();
        res
    }) {
        Some(v) => Ok(Some(T::from_bytes(&v)?)),
        None => Ok(None),
    }
}

/// Set a variable in the plug-in. This variable lives as long as the
/// plug-in is loaded. The value must have a [ToMemory] implementation.
///
/// # Arguments
///
/// * `key` - A unique string key to identify the variable
/// * `val` - The value to set. Must have a [ToMemory] implementation
///
/// # Examples
///
/// ```
/// var::set("my_u32_var", 42u32)?;
/// var::set("my_str_var", "Hello, World!")?;
/// ```
pub fn set(key: impl AsRef<str>, val: impl ToMemory) -> Result<(), Error> {
    let val = val.to_memory()?;
    let key = Memory::from_bytes(key.as_ref().as_bytes())?;
    unsafe { extism::var_set(key.offset(), val.offset()) }
    key.free();
    val.free();
    Ok(())
}

/// Removes a variable from the plug-in. This variable normally lives as long as the
/// plug-in is loaded.
///
/// # Arguments
///
/// * `key` - A unique string key to identify the variable
///
/// # Examples
///
/// ```
/// var::remove("my_var")?;
/// ```
pub fn remove(key: impl AsRef<str>) -> Result<(), Error> {
    let key = Memory::from_bytes(key.as_ref().as_bytes())?;
    unsafe { extism::var_set(key.offset(), 0) };
    key.free();
    Ok(())
}
