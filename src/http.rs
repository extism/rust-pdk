use crate::*;

#[derive(Clone, Copy)]
pub struct Http;

pub struct HttpResponse {
    memory: Memory,
}

impl HttpResponse {
    pub fn into_memory(self) -> Memory {
        self.memory
    }

    pub fn as_memory(&self) -> &Memory {
        &self.memory
    }

    pub fn body(&self) -> Vec<u8> {
        self.memory.to_vec()
    }
}

impl Http {
    pub fn request(
        &self,
        req: &extism_manifest::HttpRequest,
        body: Option<&[u8]>,
    ) -> Result<HttpResponse, serde_json::Error> {
        let enc = serde_json::to_vec(req)?;
        let req = Memory::from_bytes(&enc);
        let body = body.map(Memory::from_bytes);
        let data = body.map(|x| x.offset).unwrap_or(0);
        let res = unsafe { extism_http_request(req.offset, data) };
        let len = unsafe { extism_length(res) };
        Ok(HttpResponse {
            memory: Memory::wrap(res, len),
        })
    }
}
