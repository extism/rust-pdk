#![no_main]

use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ToBytes, FromBytes)]
#[encoding(Json)]
struct Output {
    pub count: i32,
}

#[extism_pdk::shared_fn]
pub unsafe fn hello_world(mut input: Output) -> SharedFnResult<Output> {
    info!("Hello, world!");
    input.count *= 10;
    Ok(input)
}
