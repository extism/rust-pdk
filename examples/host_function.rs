#![no_main]

use extism_pdk::*;
use serde::{Deserialize, Serialize};

const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

#[derive(Serialize, Deserialize)]
struct Output {
    pub count: i32,
}

#[host_fn]
extern "ExtismHost" {
    fn hello_world(count: Json<Output>) -> Json<Output>;
}

#[plugin_fn]
pub unsafe fn count_vowels<'a>(input: String) -> FnResult<Json<Output>> {
    let mut count = 0;
    for ch in input.chars() {
        if VOWELS.contains(&ch) {
            count += 1;
        }
    }

    let output = Output { count };
    let output = unsafe { hello_world(Json(output))? };
    Ok(output)
}
