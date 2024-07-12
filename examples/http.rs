#![no_main]

use extism_pdk::*;

#[plugin_fn]
pub fn http_get<'a>(Json(req): Json<HttpRequest>) -> FnResult<Vec<u8>> {
    info!("Request to: {}", req.url);
    let res = unwrap!(http::request::<()>(&req, None));
    Ok(res.into_vec())
}
