use crate::*;
use base64::prelude::*;
use std::sync::LazyLock;


/// If the host is appending a sentinel prefix to the host function errors
/// it will also pass that prefix to the guest, so the guest knows what to
/// trim off the front of the raw bytes. This is stored in a lazy lock because
/// it is sent once and never changed. Subsequent calls to get host function
/// errors will not need to recompute this value.
static HOST_FUNC_ERROR_PREFIX: LazyLock<Option<Vec<u8>>> = LazyLock::new(|| {
    std::env::var("EXTISM_HOST_FUNC_ERROR_PREFIX")
        .ok()
        .and_then(|p| BASE64_STANDARD.decode(p.as_bytes()).ok())
});

/// Checks to see if the most recent host function call returned an error.
/// Retrieves any error set by the host function and stripes any sentinel bytes from it.
///
/// This function interacts with the Extism kernel's `error_get` mechanism to fetch any
/// error currently stored in WASM memory. If an error exists, it determines whether the
/// error was created by a host function (using sentinel bytes in the prefix).
///
/// ### Behavior:
/// - Calls `extism::error_get` to retrieve the current error's memory offset.
/// - If no error is present (offset is `0`), returns `Ok(())`.
/// - Attempts to locate the memory at the given offset and process the error bytes:
///   - If a known sentinel prefix is found, the function strips the prefix and treats
///     the remaining bytes as the error message from the host function.
///   - If no prefix is present, the error is returned as-is.
/// - Converts the error bytes into a UTF-8 string and wraps it in an `Error::msg`.
///
/// ### Notes:
/// - Errors originating from host functions are marked with a unique sentinel prefix
///   (defined by `HOST_FUNC_ERROR_PREFIX`). This prefix is appended during serialization
///   by the host and removed during deserialization here, enabling reliable error
///   identification and processing.
pub fn get_host_func_error() -> Result<(), Error> {
    let offset = unsafe { extism::error_get() };
    if offset == 0 {
        return Ok(());
    }

    // We need to reset the error now that we've read it
    unsafe { extism::error_set(0) };

    if let Some(mem) = Memory::find(offset).map(|mem| mem.to_vec()) {
        let message = if let Some(prefix) = HOST_FUNC_ERROR_PREFIX.as_ref() {
            // Attempt to strip the sentinel prefix for host function errors
            mem.strip_prefix(prefix.as_slice())
                .map(|stripped| String::from_utf8_lossy(stripped))
                .unwrap_or_else(|| String::from_utf8_lossy(&mem))
        } else {
            // Handle the memory as a UTF-8 string if no prefix is available
            String::from_utf8_lossy(&mem)
        };

        return Err(Error::msg(message.to_string()));
    }

    Ok(())
}