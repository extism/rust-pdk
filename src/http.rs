use std::collections::HashMap;

use crate::*;

/// A HttpResponse is used to wrap the memory returned by
/// `extism_pdk::http::request`
pub struct HttpResponse {
    memory: Memory,
    status: u16,
    headers: HashMap<String, String>,
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

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn header(&self, s: impl AsRef<str>) -> Option<&str> {
        self.headers.get(s.as_ref()).map(|x| x.as_ref())
    }
}

/// Execute `HttpRequest`, if `body` is not `None` then it will be sent as the
/// request body.
pub fn request<T: ToMemory>(
    req: &extism_manifest::HttpRequest,
    body: Option<T>,
) -> Result<HttpResponse, Error> {
    let enc = serde_json::to_vec(req)?;
    let req = Memory::from_bytes(enc)?;
    let body = match body {
        Some(b) => Some(b.to_memory()?),
        None => None,
    };
    let data = body.as_ref().map(|x| x.offset()).unwrap_or(0);
    let offs = unsafe { extism::http_request(req.offset(), data) };
    let status = unsafe { extism::http_status_code() };
    let len = unsafe { extism::length_unsafe(offs) };

    let headers = unsafe { extism::http_headers() };
    let headers = if headers == 0 {
        HashMap::new()
    } else {
        if let Some(h) = Memory::find(headers) {
            let Json(j) = h.to()?;
            h.free();
            j
        } else {
            HashMap::new()
        }
    };

    Ok(HttpResponse {
        memory: Memory(MemoryHandle {
            offset: offs,
            length: len,
        }),
        status: status as u16,
        headers,
    })
}
