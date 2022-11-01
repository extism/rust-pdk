#[macro_export]
macro_rules! log {
    ($lvl:expr, $($arg:tt)+) => {{
        let fmt = format!($($arg)+);
        let memory = $crate::Memory::from_bytes(&fmt);

        unsafe {
            match $lvl {
                $crate::LogLevel::Info => {
                    $crate::bindings::extism_log_info(memory.offset);
                }
                $crate::LogLevel::Debug => {
                    $crate::bindings::extism_log_debug(memory.offset);
                }
                $crate::LogLevel::Warn => {
                    $crate::bindings::extism_log_warn(memory.offset);
                }
                $crate::LogLevel::Error => {
                    $crate::bindings::extism_log_error(memory.offset);
                }
            }
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

// #[macro_export]
// macro_rules! encoding {
//     ($name:ident, $encode:expr, $decode:expr) => {
//         pub struct $name<T>(pub T);

//         impl<T: serde::de::DeserializeOwned> $crate::Input for $name<T> {
//             fn input(d: Vec<u8>) -> Result<Self, $crate::Error> {
//                 let x = $decode(&d)?;
//                 Ok($name(x))
//             }
//         }

//         impl<T: serde::Serialize> $crate::Output for $name<T> {
//             fn output(&self) -> Result<$crate::Memory, $crate::Error> {
//                 let x = $encode(&self.0)?;
//                 Ok(Memory::from_bytes(x))
//             }
//         }
//     };
// }
