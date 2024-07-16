use crate::{header::map::HeaderMap, status::StatusCode, version::Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponseBuilder<B> {
    version: Option<Version>,
    status_code: Option<StatusCode>,
    headers: Option<HeaderMap>,
    body: Option<B>,
}

impl<B> Default for ResponseBuilder<B> {
    fn default() -> Self {
        Self {
            version: None,
            status_code: None,
            headers: None,
            body: None,
        }
    }
}

impl<B> ResponseBuilder<B> {
    pub fn version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }
    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = Some(status_code);
        self
    }
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = Some(headers);
        self
    }
    pub fn body(mut self, body: B) -> Self {
        self.body = Some(body);
        self
    }
    pub fn build(self) -> super::Response<B> {
        super::Response {
            version: self.version.unwrap_or_default(),
            status_code: self.status_code.unwrap_or_default(),
            headers: self.headers.unwrap_or_default(),
            body: self.body,
        }
    }
}
