use crate::uri::{
    get_unchecked,
    parser::{Parser, State},
    to_compact,
};
use compact_str::CompactString;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Host(CompactString);

impl Host {
    fn new(value: impl AsRef<[u8]>) -> Self {
        let compact = to_compact!(value);
        Self(compact)
    }
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        let from = {
            if parser.get_byte() == b'@' {
                parser.skip(1)
            }
            parser.position()
        };
        while parser.state() != State::Eof {
            let byte = parser.get_byte();
            if byte == b':' {
                return Some(get_unchecked!(parser, from, parser.position()));
            }
            parser.increment()
        }
        Some(get_unchecked!(parser, from))
    }
}

impl std::ops::Deref for Host {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
