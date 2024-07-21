use super::parser::{Parser, State};

/// scheme = ALPHA *( ALPHA / DIGIT / «+» / «-» / «.» )
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Scheme(std::sync::Arc<str>);

impl Scheme {
    pub(crate) fn new(value: impl AsRef<[u8]>) -> Self {
        let s = unsafe { std::str::from_utf8_unchecked(value.as_ref()) };
        Self(std::sync::Arc::from(s))
    }
    pub fn parse(uri: &str) -> Option<Self> {
        let mut parser = Parser::new(uri.as_bytes());
        while parser.state() != State::EOF {
            let &byte = unsafe { parser.get_unchecked(parser.position()) };
            if byte == b':' {
                let scheme = unsafe { parser.get_unchecked(..parser.position()) };
                return Some(Self::new(scheme))
            }
            parser.increment()
        }
        None
    }
}

impl std::ops::Deref for Scheme {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
