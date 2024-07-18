#![no_main]

use extism_pdk::*;

const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

#[derive(serde::Serialize, ToBytes)]
#[encoding(Json)]
struct TestOutput<'a> {
    pub count: i32,
    // pub config: String,
    pub a: String,
    pub b: &'a str,
}

#[plugin_fn]
pub fn count_vowels<'a>(input: String) -> FnResult<TestOutput<'a>> {
    let mut count = 0;
    for ch in input.chars() {
        if VOWELS.contains(&ch) {
            count += 1;
        }
    }

    var::set("a", var::get("a")?.unwrap_or(0u64) + 1)?;

    let a: u64 = var::get("a")?.expect("variable 'a' set");
    let a = a.to_string();
    // let config = config::get("thing")?.expect("'thing' key set in config");
    let b = "new_value";

    let output = TestOutput {
        count,
        // config,
        a,
        b,
    };

    Ok(output)
}
