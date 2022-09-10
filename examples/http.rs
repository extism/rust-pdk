#![no_main]

use extism_pdk::*;

#[no_mangle]
unsafe fn http_get() -> i32 {
    let host = Host::new();
    let s = host.input_str();
    host.log(LogLevel::Info, &format!("Request to: {}", s));
    let req = extism_manifest::HttpRequest::new(s);
    let res = unwrap!(host.http_request(&req, None));
    host.output_memory(res.memory());
    0
}
