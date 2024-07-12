use crate::*;

/// A HttpResponse is used to wrap the memory returned by
/// `extism_pdk::http::request`
pub struct HttpResponse {
    data: Vec<u8>,
    status: u16,
}

impl HttpResponse {
    pub fn status_code(&self) -> u16 {
        self.status
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }

    pub fn body(&self) -> &[u8] {
        &self.data
    }

    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, Error> {
        let x = serde_json::from_slice(&self.body())?;
        Ok(x)
    }
}

/// Execute `HttpRequest`, if `body` is not `None` then it will be sent as the
/// request body.
pub fn request<'a, T: crate::ToBytes<'a>>(
    req: &extism_manifest::HttpRequest,
    body: Option<T>,
) -> Result<HttpResponse, Error> {
    let body = match body {
        Some(b) => Some(b.to_bytes()?),
        None => None,
    };
    let req = serde_json::to_string(req)?;
    let data = body.as_ref().map(|x| read_handle(x)).unwrap_or(0);
    let len = unsafe { extism::http_request(read_handle(req), data) };
    let status = unsafe { extism::http_status_code() };
    let mut body = vec![0; len as usize];
    unsafe { extism::http_body(write_handle(&mut body)) };
    Ok(HttpResponse {
        data: body,
        status: status as u16,
    })
}
