use super::parser::{Parser, State};

/// scheme = ALPHA *( ALPHA / DIGIT / «+» / «-» / «.» )
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Scheme(std::sync::Arc<[u8]>);

impl Scheme {
    pub(crate) fn new(value: impl AsRef<[u8]>) -> Self {
        Self(std::sync::Arc::from(value.as_ref()))
    }
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        while parser.state() != State::Eof {
            let &byte = unsafe { parser.get_unchecked(parser.position()) };
            if byte == b':' {
                let scheme = unsafe { parser.get_unchecked(..parser.position()) };
                return Some(Self::new(scheme));
            }
            parser.increment()
        }
        None
    }
}

impl std::ops::Deref for Scheme {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
