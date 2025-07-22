use crate::{Error, Memory};
use extism_convert::TracingEvent;

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

/// Log a message string.
pub fn log(level: LogLevel, message: String) -> Result<(), Error> {
    let current_level = unsafe { crate::extism::get_log_level() };

    if level.to_int() >= current_level && current_level != i32::MAX {
        let memory = Memory::from_bytes(&message)?;
        memory.log(level);
    }

    Ok(())
}

/// Log a `tracing` event.
pub fn log_event(level: LogLevel, event: TracingEvent) -> Result<(), Error> {
    log(level, serde_json::to_string(&event)?)
}
