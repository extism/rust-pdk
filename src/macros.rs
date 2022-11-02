#[macro_export]
macro_rules! log {
    ($lvl:expr, $($arg:tt)+) => {{
        let fmt = format!($($arg)+);
        let memory = $crate::Memory::from_bytes(&fmt);
        memory.log($lvl)
    }}
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        $crate::log!($crate::LogLevel::Info, $($arg)+)
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {
        $crate::log!($crate::LogLevel::Debug, $($arg)+)
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => {
        $crate::log!($crate::LogLevel::Warn, $($arg)+)
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {
        $crate::log!($crate::LogLevel::Error, $($arg)+)
    }
}

#[macro_export]
macro_rules! unwrap {
    ($x:expr) => {
        match $x {
            Ok(x) => x,
            Err(e) => {
                let err = format!("{:?}", e);
                let mut mem = $crate::Memory::new(err.len());
                mem.store(err.as_bytes());
                unsafe {
                    $crate::bindings::extism_error_set(mem.offset);
                }
                return -1;
            }
        }
    };
}

#[macro_export]
macro_rules! set_var {
    ($k:expr, $($arg:tt)+) => {
        $crate::var::set($k, format!($($arg)+))
    };
}
