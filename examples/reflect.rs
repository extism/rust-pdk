#![no_main]

use extism_pdk::*;

#[shared_fn]
pub fn host_reflect(input: String) -> SharedFnResult<Vec<u8>> {
    Ok(input.to_lowercase().into_bytes())
}

#[shared_fn]
pub fn nothing() -> SharedFnResult<()> {
    Ok(())
}
