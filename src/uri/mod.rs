pub use self::{
    authority::Authority, builder::UriBuilder, fragment::Fragment, path::Path, query::Query,
    scheme::Scheme,
};

pub mod authority;
pub mod builder;
pub mod fragment;
mod parser;
pub mod path;
pub mod query;
pub mod scheme;

macro_rules! match_start_and_end {
    ($start:expr, $end:expr, $parser:expr) => {
        match ($start, $end) {
            (0, 0) => None,
            (start, 0) if start != 0 => {
                let value = unsafe { $parser.get_unchecked(start..) };
                Some(Self::new(value))
            }
            (start, end) => {
                let value = unsafe { $parser.get_unchecked(start..end) };
                Some(Self::new(value))
            }
        }
    };
}

pub(crate) use match_start_and_end;

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

#[cfg(test)]
mod tests {
    use crate::uri::Uri;
    use std::ops::Deref;

    #[test]
    fn parse_rfc_uri() {
        const URI: &str = "https://datatracker.ietf.org/some/path?query=And#fragment";
        let uri = Uri::parse_exact(URI);
        assert_eq!(uri.scheme().unwrap().deref(), "https");
        assert_eq!(uri.authority().unwrap().deref(), "datatracker.ietf.org");
        assert_eq!(uri.path().unwrap().deref(), "/some/path");
        assert_eq!(uri.query().unwrap().deref(), "query=And");
        assert_eq!(uri.fragment().unwrap().deref(), "fragment");
    }
}
