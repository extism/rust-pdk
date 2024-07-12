use crate::*;

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
    let key = read_handle(key.as_ref().as_bytes());
    let len = unsafe { extism::config_length(key) };
    if len < 0 {
        return Ok(None);
    }

    let mut value = vec![0u8; len as usize];
    unsafe {
        extism::config_read(key, write_handle(value.as_mut_slice()));
    }
    Ok(Some(String::from_utf8(value)?))
}
