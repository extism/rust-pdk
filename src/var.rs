use crate::*;

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
    let n = unsafe { extism::var_length(read_handle(key)) };
    if n < 0 {
        return Ok(None);
    }

    let mut buf = vec![0; n as usize];
    unsafe {
        extism::var_read(read_handle(key), write_handle(&mut buf));
    }

    let x = T::from_bytes_owned(&buf)?;

    Ok(Some(x))
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
pub fn set<'a>(key: impl AsRef<str>, val: impl ToBytes<'a>) -> Result<(), Error> {
    let val = val.to_bytes()?;
    unsafe {
        extism::var_write(read_handle(key.as_ref()), read_handle(val));
    }
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
    unsafe {
        extism::var_write(read_handle(key.as_ref()), 0);
    }
    Ok(())
}
