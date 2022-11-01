#![no_main]

use extism_pdk::*;
use serde::Serialize;

const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

#[derive(Serialize)]
struct TestOutput {
    pub count: i32,
    pub config: String,
    pub a: String,
}

#[function]
pub unsafe fn count_vowels(host: &mut Host, input: String) -> PluginResult<Json<TestOutput>> {
    let mut count = 0;
    for ch in input.chars() {
        if VOWELS.contains(&ch) {
            count += 1;
        }
    }

    let mut vars = host.vars();
    vars.set("a", "this is var a");

    let a = vars.get("a").expect("variable 'a' set");
    let a = String::from_utf8(a).expect("string from varible value");
    let config = host.config("thing").expect("'thing' key set in config");

    let output = TestOutput { count, config, a };
    Ok(Json(output))
}
