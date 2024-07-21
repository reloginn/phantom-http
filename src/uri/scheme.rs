use compact_str::CompactString;
use super::parser::{Parser, State};

/// scheme = ALPHA *( ALPHA / DIGIT / «+» / «-» / «.» )
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Scheme(CompactString);

impl Scheme {
    pub(crate) fn new(value: impl AsRef<[u8]>) -> Self {
        let compact = super::to_compact!(value);
        Self(compact)
    }
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        while parser.state() != State::Eof {
            let byte = parser.get_byte();
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
        self.0.as_ref()
    }
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
