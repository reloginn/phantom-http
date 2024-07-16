pub use self::{
    authority::Authority, builder::UriBuilder, fragment::Fragment, path::Path, query::Query,
    scheme::Scheme,
};

pub mod authority;
pub mod builder;
pub mod fragment;
pub mod path;
pub mod query;
pub mod scheme;

/// See [RFC3986](https://datatracker.ietf.org/doc/html/rfc3986) for more details.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Uri {
    pub scheme: Option<Scheme>,
    pub authority: Option<Authority>,
    pub path: Option<Path>,
    pub query: Option<Query>,
    pub fragment: Option<Fragment>,
}

impl Uri {
    pub fn parse_exact(uri: impl AsRef<str>) -> Self {
        let uri = uri.as_ref();
        Self {
            scheme: Scheme::parse(uri),
            authority: Authority::parse(uri),
            path: Path::parse(uri),
            query: Query::parse(uri),
            fragment: Fragment::parse(uri),
        }
    }
    pub fn scheme(&self) -> Option<&Scheme> {
        self.scheme.as_ref()
    }
    pub fn authority(&self) -> Option<&Authority> {
        self.authority.as_ref()
    }
    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref()
    }
    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }
    pub fn fragment(&self) -> Option<&Fragment> {
        self.fragment.as_ref()
    }
    pub fn builder<'a>() -> UriBuilder<'a> {
        UriBuilder::default()
    }
}
