mod macros;

pub mod bindings;
mod host;
mod input;
mod memory;
mod output;
mod vars;

pub use anyhow::Error;
pub(crate) use bindings::*;
pub use extism_manifest::HttpRequest;
pub use extism_pdk_derive::function;
pub use host::{Host, HttpResponse};
pub use input::Input;
pub use memory::Memory;
pub use output::Output;
pub use vars::Vars;

pub use serde_json as json;

encoding!(Json, serde_json::to_vec, serde_json::from_slice);

pub type PluginResult<T> = Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Debug,
    Warn,
    Error,
}
