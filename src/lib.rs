mod macros;

pub mod bindings;
pub mod config;
mod from_bytes;
mod memory;
mod to_memory;
pub mod var;

#[cfg(feature = "http")]
pub mod http;

pub use anyhow::Error;
pub(crate) use bindings::*;
pub use extism_pdk_derive::{encoding, function};
pub use from_bytes::FromBytes;
pub use memory::Memory;
pub use to_memory::ToMemory;

#[cfg(feature = "http")]
pub use extism_manifest::HttpRequest;

#[cfg(feature = "http")]
pub use http::HttpResponse;

/// The return type of a plugin function
pub type PluginResult<T> = Result<T, Error>;

/// Logging levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Debug,
    Warn,
    Error,
}

/// Re-export of `serde_json`
pub use serde_json as json;

use crate as extism_pdk;
/// JSON encoding
#[encoding(serde_json::to_vec, serde_json::from_slice)]
pub struct Json;

/// Get input from host
pub fn input<T: FromBytes>() -> Result<T, Error> {
    unsafe { T::from_bytes(extism_load_input()) }
}

/// Set output for host
pub fn output(data: impl ToMemory) -> Result<(), Error> {
    data.to_memory()?.set_output();
    Ok(())
}
