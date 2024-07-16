use crate::{header::map::HeaderMap, method::Method, uri::Uri, version::Version};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestBuilder<B> {
    method: Option<Method>,
    uri: Option<Uri>,
    version: Option<Version>,
    headers: Option<HeaderMap>,
    body: Option<B>,
}

impl<B> Default for RequestBuilder<B> {
    fn default() -> Self {
        Self {
            method: None,
            uri: None,
            version: None,
            headers: None,
            body: None,
        }
    }
}

impl<B> RequestBuilder<B> {
    pub fn method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }
    pub fn uri(mut self, uri: Uri) -> Self {
        self.uri = Some(uri);
        self
    }
    pub fn version(mut self, version: Version) -> Self {
        self.version = Some(version);
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
    pub fn build(self) -> super::Request<B> {
        super::Request {
            method: self.method.unwrap_or_default(),
            uri: self.uri.unwrap_or_default(),
            version: self.version.unwrap_or_default(),
            headers: self.headers.unwrap_or_default(),
            body: self.body,
        }
    }
}
