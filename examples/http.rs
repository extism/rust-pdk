#![no_main]

use extism_pdk::*;

#[plugin_fn]
pub fn http_get(Json(req): Json<HttpRequest>) -> FnResult<Memory> {
    trace!("HTTP Request: {:?}", req);
    info!("Request to: {}", req.url);
    let res = http::request::<()>(&req, None)?;
    Ok(res.into_memory())
}
