#![no_main]

use extism_pdk::*;

#[no_mangle]
unsafe fn http_get() -> i32 {
    let host = Host::new();
    let s = host.input_str();

    let req = extism_manifest::HttpRequest::new(s);
    let res = host.http_request(&req, None).unwrap();
    host.output_memory(res.memory());
    0
}
