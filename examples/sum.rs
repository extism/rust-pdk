#![no_main]

use extism_pdk::*;

#[derive(serde::Deserialize, FromBytes)]
#[encoding(Json)]
struct Add {
    a: u32,
    b: u32,
}

#[derive(serde::Serialize, ToBytes)]
#[encoding(Json)]
struct Sum {
    sum: u32,
}

#[plugin_fn]
pub fn add(add: Add) -> FnResult<Sum> {
    let sum = Sum { sum: add.a + add.b };
    Ok(sum)
}
