#![no_main]

use extism_pdk::*;
use serde::Serialize;

const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

#[derive(Serialize)]
struct TestOutput<'a> {
    pub count: i32,
    pub config: String,
    pub a: String,
    pub b: &'a str,
}

#[plugin_fn]
pub unsafe fn count_vowels<'a>(input: String) -> FnResult<Json<TestOutput<'a>>> {
    let mut count = 0;
    for ch in input.chars() {
        if VOWELS.contains(&ch) {
            count += 1;
        }
    }

    set_var!("a", "this is var a")?;

    let a = var::get("a")?.expect("variable 'a' set");
    let a = String::from_utf8(a).expect("string from varible value");
    let config = config::get("thing").expect("'thing' key set in config");
    let b = "new_value";

    let output = TestOutput {
        count,
        config,
        a,
        b,
    };
    Ok(Json(output))
}
