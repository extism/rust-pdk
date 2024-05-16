#![no_main]

use extism_pdk::*;

#[export_fn]
pub fn reflect(input: String) -> ExportResult<Vec<u8>> {
    Ok(input.to_lowercase().into_bytes())
}
