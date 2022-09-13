#![no_main]

use extism_pdk::*;
use serde::Serialize;
use serde_json;

const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

#[derive(Serialize)]
struct TestOutput {
    pub count: i32,
    pub config: String,
    pub a: String,
}

#[no_mangle]
unsafe fn count_vowels() -> i32 {
    let host = Host::new();
    let s = host.input_str();

    let mut count = 0;
    for ch in s.chars() {
        if VOWELS.contains(&ch) {
            count += 1;
        }
    }

    let mut vars = host.vars();
    vars.set("a", "this is var a");

    let a = vars.get("a").expect("variable 'a' set").into_inner();
    let a = String::from_utf8(a).expect("string from varible value");
    let config = host
        .config("thing")
        .expect("'thing' key set in config")
        .into_inner();

    let output = TestOutput { count, config, a };

    host.output(&serde_json::to_string_pretty(&output).expect("json serialize output"));
    0
}
