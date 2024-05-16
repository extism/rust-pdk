#![no_main]

use extism_pdk::*;

#[export_fn]
pub fn reflect(input: Vec<u8>) -> ExportResult<Vec<u8>> {
    Ok(input)
}
