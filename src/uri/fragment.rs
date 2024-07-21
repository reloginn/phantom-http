use super::parser::{Parser, State};

/// fragment    = *( pchar / "/" / "?" )
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fragment(std::sync::Arc<[u8]>);

impl Fragment {
    pub(crate) fn new(value: impl AsRef<[u8]>) -> Self {
        Self(std::sync::Arc::from(value.as_ref()))
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
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}
