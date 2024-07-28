use super::{
    macros::{get_unchecked, to_compact},
    parser::{Parser, State},
};
use compact_str::CompactString;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Repr {
    Http,
    Https,
    Other(CompactString),
}

/// scheme = ALPHA *( ALPHA / DIGIT / «+» / «-» / «.» )
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Scheme(Repr);

impl Scheme {
    pub(crate) fn new(value: impl AsRef<[u8]>) -> Self {
        let repr = match value.as_ref() {
            b"http" => Repr::Http,
            b"https" => Repr::Https,
            other => Repr::Other(to_compact!(other)),
        };
        Self(repr)
    }
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        while parser.state() != State::Eof {
            let byte = parser.get_byte();
            if byte == b':' {
                return Some(get_unchecked!(parser, parser.position(), !));
            }
            parser.increment()
        }
        None
    }
}

impl std::ops::Deref for Scheme {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match &self.0 {
            Repr::Http => "http",
            Repr::Https => "https",
            Repr::Other(other) => other.as_ref(),
        }
    }
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
