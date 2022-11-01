mod macros;

pub mod bindings;
mod config;
mod host;
pub mod http;
mod input;
mod memory;
mod output;
mod vars;

pub use anyhow::Error;
pub(crate) use bindings::*;
pub use config::Config;
pub use extism_manifest::HttpRequest;
pub use extism_pdk_derive::{encoding, function};
pub use host::Host;
pub use http::HttpResponse;
pub use input::Input;
pub use memory::Memory;
pub use output::Output;
pub use vars::Vars;

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
