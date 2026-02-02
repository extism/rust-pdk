#![no_main]

//! This example demonstrates the Rust 1.82+ `unsafe extern` syntax with `safe fn`.
//!
//! In `unsafe extern` blocks, you can mark individual functions as `safe` to indicate
//! they are safe to call without an unsafe block. Functions without the `safe` qualifier
//! are implicitly unsafe.

use extism_pdk::*;
use serde::{Deserialize, Serialize};

const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
struct Output {
    pub count: i32,
}

// Using Rust 1.82+ unsafe extern syntax with safe fn
#[host_fn("extism:host/user")]
unsafe extern "ExtismHost" {
    // This function is marked safe - the generated wrapper is safe to call
    safe fn hello_world(count: Output) -> Output;
}

#[plugin_fn]
pub fn count_vowels<'a>(input: String) -> FnResult<Output> {
    let mut count = 0;
    for ch in input.chars() {
        if VOWELS.contains(&ch) {
            count += 1;
        }
    }

    let output = Output { count };
    // No unsafe block needed because hello_world is marked as `safe fn`
    let output = hello_world(output)?;
    Ok(output)
}
