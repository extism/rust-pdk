use crate::*;

/// A HttpResponse is used to wrap the memory returned by
/// `extism_pdx::http::request`
pub struct HttpResponse {
    memory: Memory,
    status: u16,
}

impl HttpResponse {
    pub fn into_memory(self) -> Memory {
        self.memory
    }

    pub fn status_code(&self) -> u16 {
        self.status
    }

    pub fn as_memory(&self) -> &Memory {
        &self.memory
    }

    pub fn body(&self) -> Vec<u8> {
        self.memory.to_vec()
    }

    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, Error> {
        let x = serde_json::from_slice(&self.body())?;
        Ok(x)
    }
}

/// Execute `HttpRequest`, if `body` is not `None` then it will be sent as the
/// request body.
pub fn request<T: ToMemory>(
    req: &extism_manifest::HttpRequest,
    body: Option<T>,
) -> Result<HttpResponse, Error> {
    let enc = serde_json::to_vec(req)?;
    let req = Memory::from_bytes(enc);
    let body = match body {
        Some(b) => Some(b.to_memory()?),
        None => None,
    };
    let data = body.as_ref().map(|x| x.offset).unwrap_or(0);
    let offs = unsafe { extism_http_request(req.offset, data) };
    let status = unsafe { extism_http_status_code() };
    let len = unsafe { extism_length(offs) };
    Ok(HttpResponse {
        memory: Memory::wrap(offs, len, true),
        status: status as u16,
    })
}
