use super::{
    macros::{match_start_and_end, to_compact},
    parser::{Parser, State},
};
use compact_str::CompactString;
use components::AuthorityComponents;

pub mod components;
pub mod host;
pub mod port;
pub mod userinfo;

/// authority = [ userinfo «@» ] host [ «:» port ]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authority(CompactString);

impl Authority {
    pub(crate) fn new(value: impl AsRef<[u8]>) -> Self {
        let compact = to_compact!(value);
        Self(compact)
    }
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        let start = 'start: {
            let mut colon = false;
            let mut slash = false;
            while parser.state() != State::Eof {
                let byte = parser.get_byte();
                if byte == b':' {
                    colon = true;
                }
                if byte == b'/' && colon {
                    slash = true;
                }
                if byte == b'/' && slash {
                    parser.skip(2);
                    break 'start parser.position();
                }
                parser.increment()
            }
            0
        };
        let end = 'end: {
            while parser.state() != State::Eof {
                let byte = parser.get_byte();
                if byte == b'/' || byte == b'?' || byte == b'#' && start != 0 {
                    break 'end parser.position();
                }
                parser.increment()
            }
            0
        };
        match_start_and_end!(start, end, parser)
    }
    pub fn to_components(&self) -> AuthorityComponents {
        AuthorityComponents::new(self)
    }
}

impl std::ops::Deref for Authority {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl std::fmt::Display for Authority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
