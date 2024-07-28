pub use self::{
    authority::Authority, builder::UriBuilder, fragment::Fragment, path::Path, query::Query,
    scheme::Scheme,
};

pub mod authority;
pub mod builder;
pub mod fragment;
pub(crate) mod macros;
pub(crate) mod parser;
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
    pub fn parse_exact(uri: impl AsRef<[u8]>) -> Self {
        let mut parser = parser::Parser::new(uri.as_ref());
        Self {
            scheme: Scheme::parse(&mut parser),
            authority: Authority::parse(&mut parser),
            path: Path::parse(&mut parser),
            query: Query::parse(&mut parser),
            fragment: Fragment::parse(&mut parser),
        }
    }
    pub fn scheme(&self) -> Option<&Scheme> {
        self.scheme.as_ref()
    }
    pub fn scheme_str(&self) -> Option<&str> {
        self.scheme.as_deref()
    }
    pub fn authority(&self) -> Option<&Authority> {
        self.authority.as_ref()
    }
    pub fn authority_str(&self) -> Option<&str> {
        self.authority.as_deref()
    }
    pub fn path(&self) -> Option<&Path> {
        self.path.as_ref()
    }
    pub fn path_str(&self) -> Option<&str> {
        self.path.as_deref()
    }
    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }
    pub fn query_str(&self) -> Option<&str> {
        self.query.as_deref()
    }
    pub fn fragment(&self) -> Option<&Fragment> {
        self.fragment.as_ref()
    }
    pub fn fragment_str(&self) -> Option<&str> {
        self.fragment.as_deref()
    }
    pub fn builder<'a>() -> UriBuilder<'a> {
        UriBuilder::default()
    }
}

impl std::str::FromStr for Uri {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse_exact(s))
    }
}

#[cfg(test)]
mod tests {
    use super::Uri;

    #[test]
    fn parse_rfc3986_uri() {
        const URI: &str = "https://datatracker.ietf.org/doc/html/rfc3986";
        let uri = Uri::parse_exact(URI);
        assert_eq!(uri.scheme_str(), Some("https"));
        assert_eq!(uri.authority_str(), Some("datatracker.ietf.org"));
        assert_eq!(uri.path_str(), Some("/doc/html/rfc3986"));
        assert_eq!(uri.query(), None);
        assert_eq!(uri.fragment(), None);
    }

    #[test]
    fn parse_authority_to_components() {
        const URI: &str = "https://userinfo@host.org:443";
        let uri = Uri::parse_exact(URI);
        let components = uri
            .authority()
            .map(|authority| authority.to_components())
            .unwrap();
        assert_eq!(components.userinfo_str(), Some("userinfo"));
        assert_eq!(components.host_str(), Some("host.org"));
        assert_eq!(components.port_u16(), Some(443));
    }
}
