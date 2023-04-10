#![no_main]

use extism_pdk::*;

#[plugin_fn]
pub fn http_get(Json(req): Json<HttpRequest>) -> FnResult<String> {
    info!("Request to: {}", req.url);
    let req1 = &HttpRequest::new(req.url).with_method("POST");
    let res = http::request::<String>(req1, None)?;
    Ok(String::from("test"))
}
