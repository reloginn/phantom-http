use super::{
    match_start_and_end,
    parser::{Parser, State},
};
use compact_str::CompactString;

/// query       = *( pchar / "/" / "?" )
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query(CompactString);

impl Query {
    pub(crate) fn new(value: impl AsRef<[u8]>) -> Self {
        let compact = super::to_compact!(value);
        Self(compact)
    }
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        let start = 'start: {
            while parser.state() != State::Eof {
                let byte = parser.get_byte();
                if byte == b'?' {
                    parser.skip(1);
                    break 'start parser.position();
                }
                parser.increment()
            }
            0
        };
        let end = 'end: {
            while parser.state() != State::Eof {
                let byte = parser.get_byte();
                if byte == b'#' && start != 0 {
                    break 'end parser.position();
                }
                parser.increment()
            }
            0
        };
        match_start_and_end!(start, end, parser)
    }
}

impl std::ops::Deref for Query {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}
