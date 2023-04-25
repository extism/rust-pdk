#![no_main]

use extism_pdk::*;

#[plugin_fn]
pub fn http_get(Json(req): Json<HttpRequest>) -> FnResult<Memory> {
    info!("Request to: {}", req.url);
    let res = http::request::<String>(&req, None)?;
    Ok(res.into_memory().keep())
}
