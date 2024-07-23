use crate::uri::{
    get_unchecked,
    parser::{Parser, State},
    to_compact,
};
use compact_str::CompactString;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserInfo(CompactString);

impl UserInfo {
    fn new(value: impl AsRef<[u8]>) -> Self {
        let compact = to_compact!(value);
        Self(compact)
    }
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        while parser.state() != State::Eof {
            let byte = parser.get_byte();
            if byte == b'@' {
                return Some(get_unchecked!(parser, parser.position(), !));
            }
            parser.increment()
        }
        None
    }
}

impl std::ops::Deref for UserInfo {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl std::fmt::Display for UserInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
