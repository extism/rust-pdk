mod macros;

pub mod bindings;
pub mod config;
mod host;
mod input;
mod memory;
mod to_memory;
pub mod var;

#[cfg(feature = "http")]
pub mod http;

pub use anyhow::Error;
pub(crate) use bindings::*;
pub use extism_pdk_derive::{encoding, function};
pub use host::*;
pub use input::Input;
pub use memory::Memory;
pub use to_memory::ToMemory;

#[cfg(feature = "http")]
pub use extism_manifest::HttpRequest;

#[cfg(feature = "http")]
pub use http::HttpResponse;

pub type PluginResult<T> = Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Debug,
    Warn,
    Error,
}

pub use serde_json as json;

use crate as extism_pdk;

#[encoding(serde_json::to_vec, serde_json::from_slice)]
pub struct Json;
