use crate::uri::{get_unchecked, parser::Parser, to_compact};
use compact_str::CompactString;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Port(CompactString);

impl Port {
    fn new(value: impl AsRef<[u8]>) -> Self {
        let compact = to_compact!(value);
        Self(compact)
    }
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        if parser.get_byte() == b':' {
            parser.skip(1);
            Some(get_unchecked!(parser, parser.position()))
        } else {
            None
        }
    }
}

impl std::ops::Deref for Port {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl std::fmt::Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
