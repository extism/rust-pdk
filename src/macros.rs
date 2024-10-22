#[macro_export]
macro_rules! log {
    ($lvl:expr, $($arg:tt)+) => {{
        let level = unsafe { $crate::extism::get_log_level() };
        if $lvl.to_int() >= level && level != i32::MAX  {
            let fmt = format!($($arg)+);
            let memory = $crate::Memory::from_bytes(&fmt).unwrap();
            memory.log($lvl)
        }
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
macro_rules! trace {
    ($($arg:tt)+) => {
        $crate::log!($crate::LogLevel::Trace, $($arg)+)
    }
}

#[macro_export]
macro_rules! unwrap {
    ($x:expr) => {
        match $x {
            Ok(x) => x,
            Err(e) => {
                let err = format!("{:?}", e);
                let mut mem = $crate::Memory::from_bytes(&err).unwrap();
                unsafe {
                    $crate::extism::error_set(mem.offset());
                }
                return -1;
            }
        }
    };
}

#[macro_export]
macro_rules! set_var {
    ($k:expr, $($arg:tt)+) => {
        $crate::var::set($k, &format!($($arg)+))
    };
}
