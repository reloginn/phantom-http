#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct UriBuilder<'a> {
    scheme: Option<&'a str>,
    authority: Option<&'a str>,
    path: Option<&'a str>,
    query: Option<&'a str>,
    fragment: Option<&'a str>,
}

impl<'a> UriBuilder<'a> {
    pub const fn scheme(mut self, scheme: &'a str) -> Self {
        self.scheme = Some(scheme);
        self
    }
    pub const fn authority(mut self, authority: &'a str) -> Self {
        self.authority = Some(authority);
        self
    }
    pub const fn path(mut self, path: &'a str) -> Self {
        self.path = Some(path);
        self
    }
    pub const fn query(mut self, query: &'a str) -> Self {
        self.query = Some(query);
        self
    }
    pub const fn fragment(mut self, fragment: &'a str) -> Self {
        self.fragment = Some(fragment);
        self
    }
    pub fn build(self) -> super::Uri {
        super::Uri {
            scheme: self.scheme.map(super::Scheme::new),
            authority: self.authority.map(super::Authority::new),
            path: self.path.map(super::Path::new),
            query: self.query.map(super::Query::new),
            fragment: self.fragment.map(super::Fragment::new),
        }
    }
}
