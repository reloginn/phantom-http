use crate::{header::map::HeaderMap, status::StatusCode, version::Version};
use builder::ResponseBuilder;

pub mod builder;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response<B> {
    version: Version,        // -------|
    status_code: StatusCode, //        |---------- HEAD
    headers: HeaderMap,
    body: Option<B>,
}

impl<B> Response<B> {
    pub fn version(&self) -> Version {
        self.version
    }
    pub fn status_code(&self) -> StatusCode {
        self.status_code
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
    pub fn builder() -> ResponseBuilder<B> {
        ResponseBuilder::default()
    }
}
