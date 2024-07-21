use super::parser::{Parser, State};

/// authority = [ userinfo «@» ] host [ «:» port ]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authority(std::sync::Arc<[u8]>);

impl Authority {
    pub(crate) fn new(value: impl AsRef<[u8]>) -> Self {
        Self(std::sync::Arc::from(value.as_ref()))
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
        super::match_start_and_end!(start, end, parser)
    }
}

impl std::ops::Deref for Authority {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl std::fmt::Display for Authority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
