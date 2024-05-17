#![no_main]

use extism_pdk::*;

#[export_fn]
pub fn host_reflect(input: String) -> ExportResult<Vec<u8>> {
    Ok(input.to_lowercase().into_bytes())
}

#[export_fn]
pub fn nothing() -> ExportResult<()> {
    Ok(())
}
