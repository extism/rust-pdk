#![no_main]

use extism_pdk::*;

#[plugin_fn]
pub fn http_get(Json(req): Json<HttpRequest>) -> FnResult<HttpResponse> {
    info!("Request to: {}", req.url);
    let res = http::request::<()>(&req, None)?;
    Ok(res)
}
