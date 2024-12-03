use crate::*;

/// Checks to see if the most recent host function call returned an error.
pub fn get_host_func_error() -> Result<(), Error> {
    let offset = unsafe { extism::host_func_get_error() };
    if offset == 0 {
        return Ok(());
    }
    match Memory::find(offset)
        .map(|mem| mem.to_string().ok())
        .flatten()
    {
        None => Ok(()),
        Some(mem) => Err(Error::msg(mem.to_string())),
    }
}
