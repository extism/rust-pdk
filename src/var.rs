use std::collections::BTreeMap;

use crate::*;

static mut VARS: BTreeMap<String, Vec<u8>> = BTreeMap::new();

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
    let key = key.as_ref();
    if let Some(buf) = unsafe { VARS.get(key) } {
        let x = T::from_bytes_owned(&buf)?;

        return Ok(Some(x));
    }

    Ok(None)
}

/// Set a variable in the plug-in. This variable lives as long as the
/// plug-in is loaded.
///
/// # Arguments
///
/// * `key` - A unique string key to identify the variable
/// * `val` - The value to set.
///
/// # Examples
///
/// ```
/// var::set("my_u32_var", 42u32)?;
/// var::set("my_str_var", "Hello, World!")?;
/// ```
pub fn set<'a>(key: impl Into<String>, val: impl ToBytes<'a>) -> Result<(), Error> {
    let key = key.into();
    let val = val.to_bytes()?;

    unsafe { VARS.insert(key, val.as_ref().to_vec()) };
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
    let key = key.as_ref();
    unsafe {
        VARS.remove(key);
    }
    Ok(())
}
