#![no_main]

use std::collections::HashMap;

use extism_pdk::*;

#[plugin_fn]
pub fn http_get(Json(req): Json<HttpRequest>) -> FnResult<Json<HashMap<String, String>>> {
    info!("Request to: {}", req.url);
    let res = http::request::<()>(&req, None)?;
    Ok(Json(res.headers().clone()))
}
