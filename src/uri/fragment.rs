use super::parser::{Parser, State};
use compact_str::CompactString;

/// fragment    = *( pchar / "/" / "?" )
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fragment(CompactString);

impl Fragment {
    pub(crate) fn new(value: impl AsRef<[u8]>) -> Self {
        let compact = super::to_compact!(value);
        Self(compact)
    }
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        while parser.state() != State::Eof {
            let byte = parser.get_byte();
            if byte == b'#' {
                parser.skip(1);
                let fragment = unsafe { parser.get_unchecked(parser.position()..) };
                return Some(Self::new(fragment));
            }
            parser.increment()
        }
        None
    }
}

impl std::ops::Deref for Fragment {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
