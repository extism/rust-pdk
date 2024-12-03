#![allow(clippy::missing_safety_doc)]

#[cfg(target_arch = "wasm32")]
pub use std::arch::wasm32::v128;

mod macros;

pub mod extism;
pub mod memory;
mod to_memory;

/// Functions to read plug-in config
pub mod config;

/// Functions to manipulate plug-in variables
pub mod var;

#[cfg(feature = "http")]
/// Types and functions for making HTTP requests
pub mod http;

/// Functions and utilities for working with host function errors
mod error;
pub use error::get_host_func_error;

pub use anyhow::Error;
pub use extism_convert::*;
pub use extism_convert::{FromBytes, FromBytesOwned, ToBytes};
pub use extism_pdk_derive::{host_fn, plugin_fn, shared_fn};
pub use memory::{Memory, MemoryPointer};
pub use to_memory::ToMemory;

#[cfg(feature = "http")]
/// HTTP request type
pub use extism_manifest::HttpRequest;

#[cfg(feature = "http")]
/// HTTP response type
pub use http::HttpResponse;

/// The return type of a plugin function
pub type FnResult<T> = Result<T, WithReturnCode<Error>>;

/// The return type of a `shared_fn`
pub type SharedFnResult<T> = Result<T, Error>;

/// Logging levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub const fn to_int(self) -> i32 {
        match self {
            LogLevel::Trace => 0,
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
        }
    }
}

/// Re-export of `serde_json`
pub use serde_json as json;

/// Base64 string
pub struct Base64(pub String);

/// Get input bytes from host
pub fn input_bytes() -> Vec<u8> {
    unsafe { extism::load_input() }
}

/// Get input bytes from host and convert into `T`
pub fn input<T: FromBytesOwned>() -> Result<T, Error> {
    let data = input_bytes();
    T::from_bytes_owned(&data)
}

/// Set output for host
pub fn output<T: ToMemory>(data: T) -> Result<(), Error> {
    let data = data.to_memory()?;
    data.set_output();
    Ok(())
}

pub struct WithReturnCode<T>(pub T, pub i32);

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
