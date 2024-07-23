#![allow(clippy::missing_safety_doc)]

#[cfg(target_arch = "wasm32")]
pub use std::arch::wasm32::v128;

mod macros;

pub mod extism;

/// Functions to read plug-in config
pub mod config;

/// Functions to manipulate plug-in variables
pub mod var;

#[cfg(feature = "http")]
/// Types and functions for making HTTP requests
pub mod http;
pub use anyhow::Error;
pub use extism_convert::*;
pub use extism_convert::{FromBytes, FromBytesOwned, ToBytes};
pub use extism_pdk_derive::{host_fn, plugin_fn, shared_fn};

#[cfg(feature = "http")]
/// HTTP request type
pub use extism_manifest::HttpRequest;

#[cfg(feature = "http")]
/// HTTP response type
pub use http::HttpResponse;

/// The return type of a plugin function
pub type FnResult<T> = Result<T, Error>;

/// Logging levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

#[doc(hidden)]
pub type Handle = u64;

#[doc(hidden)]
pub fn write_handle(mut x: impl AsMut<[u8]>) -> Handle {
    let x = x.as_mut();
    let offs = (x.as_mut_ptr() as u64) << 32;
    offs | x.len() as u64
}

#[doc(hidden)]
pub fn read_handle(x: impl AsRef<[u8]>) -> Handle {
    let x = x.as_ref();
    let offs = (x.as_ptr() as u64) << 32;
    offs | x.len() as u64
}

/// Re-export of `serde_json`
pub use serde_json as json;

/// Base64 string
pub struct Base64(pub String);

/// Get input bytes from host
pub fn read_input(buf: &mut [u8]) -> Option<usize> {
    let n = unsafe { extism::read(extism::Stream::Input, write_handle(buf)) };
    if n < 0 {
        return None;
    }

    Some(n as usize)
}

pub fn input<T: FromBytesOwned>() -> Result<T, Error> {
    let buf = &mut [0; 1024];
    let mut dest = Vec::new();

    while let Some(n) = read_input(buf) {
        dest.extend_from_slice(&mut buf[..n]);
    }

    T::from_bytes_owned(&dest)
}

/// Set output for host
pub fn write_output(data: &[u8]) -> Result<(), Error> {
    let n = unsafe { extism::write(extism::Stream::Output, read_handle(data)) };
    if n < 0 {
        anyhow::bail!("Writing to closed output pipe")
    }

    Ok(())
}

pub fn output<'a, T: ToBytes<'a>>(x: T) -> Result<(), Error> {
    let b = x.to_bytes()?;
    write_output(b.as_ref())?;
    Ok(())
}

pub fn error(msg: impl AsRef<str>) -> ! {
    unsafe { extism::error(read_handle(msg.as_ref())) }
}

pub fn log(level: LogLevel, msg: impl AsRef<str>) {
    unsafe {
        extism::log(level as i32, read_handle(msg.as_ref()));
    }
}
