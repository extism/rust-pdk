#![no_main]

use extism_pdk::*;

#[function]
pub fn http_get(Json(req): Json<HttpRequest>) -> FuncResult<HttpResponse> {
    info!("Request to: {}", req.url);
    let res = http::request::<()>(&req, None)?;
    Ok(res)
}
