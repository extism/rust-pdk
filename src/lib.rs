#[cfg(target_arch = "wasm32")]
pub use std::arch::wasm32::v128;

mod macros;

pub mod bindings;
pub mod config;
mod from_bytes;
pub mod memory;
mod to_memory;
pub mod var;

#[cfg(feature = "http")]
/// Types and functions for making HTTP requests
pub mod http;

pub use anyhow::Error;
pub(crate) use bindings::*;
pub use extism_convert::*;
pub use extism_pdk_derive::{host_fn, plugin_fn};
pub use memory::Memory;
pub use to_memory::ToMemory;

#[cfg(feature = "http")]
/// HTTP request type
pub use extism_manifest::HttpRequest;

#[cfg(feature = "http")]
/// HTTP response type
pub use http::HttpResponse;

/// The return type of a plugin function
pub type FnResult<T> = Result<T, WithReturnCode<Error>>;

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

/// Base64 string
pub struct Base64(pub String);

/// Get input from host
pub fn input<'a, T: FromBytes<'a>>() -> Result<T, Error> {
    unsafe { T::from_bytes(&extism_load_input()) }
}

/// Set output for host
pub fn output<'a>(data: impl ToBytes<'a>) -> Result<(), Error> {
    let data = data.to_bytes()?;
    let data = data.as_ref();
    data.to_memory()?.set_output();
    Ok(())
}

pub struct WithReturnCode<T>(T, i32);

impl<E: Into<Error>> From<E> for WithReturnCode<Error> {
    fn from(value: E) -> Self {
        WithReturnCode::new(value.into(), -1)
    }
}

impl std::fmt::Debug for WithReturnCode<Error> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: ToMemory> ToMemory for WithReturnCode<T> {
    fn to_memory(&self) -> Result<Memory, Error> {
        self.0.to_memory()
    }

    fn status(&self) -> i32 {
        self.1
    }
}

impl<T> WithReturnCode<T> {
    pub fn new(x: T, status: i32) -> Self {
        WithReturnCode(x, status)
    }
}
