use crate::{header::map::HeaderMap, method::Method, uri::Uri, version::Version};
use builder::RequestBuilder;

pub mod builder;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request<B> {
    method: Method,   // --------------|
    uri: Uri,         // --------------|--- HEAD
    version: Version, // --------------|
    headers: HeaderMap,
    body: Option<B>,
}

impl<B> Request<B> {
    pub fn method(&self) -> Method {
        self.method
    }
    pub fn uri(&self) -> &Uri {
        &self.uri
    }
    pub fn version(&self) -> Version {
        self.version
    }
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }
    pub fn body(&self) -> Option<&B> {
        self.body.as_ref()
    }
    pub fn body_mut(&mut self) -> Option<&mut B> {
        self.body.as_mut()
    }
    pub fn builder() -> RequestBuilder<B> {
        RequestBuilder::default()
    }
}
