#![no_main]

use extism_pdk::*;
use serde::{Deserialize, Serialize};

const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
struct Output {
    pub count: i32,
}

#[host_fn("extism:host/user")]
extern "ExtismHost" {
    fn hello_world(count: Output) -> Output;
}

#[plugin_fn]
pub unsafe fn count_vowels<'a>(input: String) -> FnResult<Output> {
    let mut count = 0;
    for ch in input.chars() {
        if VOWELS.contains(&ch) {
            count += 1;
        }
    }

    let output = Output { count };
    let output = unsafe { hello_world(output)? };
    Ok(output)
}
