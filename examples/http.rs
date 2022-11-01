#![no_main]

use extism_pdk::*;

#[function]
pub fn http_get(host: &mut Host, Json(req): Json<HttpRequest>) -> PluginResult<HttpResponse> {
    info!("Request to: {}", req.url);
    let res = host.http_request(&req, None)?;
    Ok(res)
}
