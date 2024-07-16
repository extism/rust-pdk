#![no_main]

use extism_pdk::*;
use serde::{Deserialize, Serialize};

const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
struct Output {
    pub count: i32,
}

#[link(wasm_import_module = "extism:host/user")]
extern "C" {
    #[link_name = "hello_world"]
    fn hello_world(count: u64) -> u64;
}

#[plugin_fn]
pub unsafe fn count_vowels<'a>(input: String) -> FnResult<()> {
    let mut count = 0;
    for ch in input.chars() {
        if VOWELS.contains(&ch) {
            count += 1;
        }
    }

    let c = unsafe { hello_world(count as u64) };
    output(c)
}
