use super::parser::{Parser, State};

/// authority = [ userinfo «@» ] host [ «:» port ]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authority(std::sync::Arc<[u8]>);

impl Authority {
    pub(crate) fn new(value: impl AsRef<[u8]>) -> Self {
        Self(std::sync::Arc::from(value.as_ref()))
    }
    pub fn parse(uri: &str) -> Option<Self> {
        let mut parser = Parser::new(uri.as_bytes());
        let mut start = 'start: {
            while parser.state() != State::EOF {
                let &byte = unsafe { parser.get_unchecked(parser.position()) };
                if byte == b':' {
                    parser.increment();
                    let &slash = unsafe { parser.get_unchecked(parser.position()) };
                    if slash == b'/' {
                        parser.increment();
                        let &slash = unsafe { parser.get_unchecked(parser.position()) };
                        if slash == b'/' {
                            parser.increment();
                            break 'start parser.position();
                        } else {
                            parser.decrement()
                        }
                    } else {
                        parser.decrement()
                    }
                }
                parser.increment()
            }
            0
        };
        let mut end = 'end: {
            while parser.state() != State::EOF {
                let &byte = unsafe { parser.get_unchecked(parser.position()) };
                if byte == b'/' || byte == b'?' || byte == b'#' {
                    break 'end parser.position()
                }
                parser.increment()
            }
            0
        };
        match (start, end) {
            (0, 0) => None,
            (start, 0) if start != 0 => {
                let authority = unsafe { parser.get_unchecked(start..) };
                Some(Self::new(authority))
            }
            (start, end) => {
                let authority = unsafe { parser.get_unchecked(start..end) };
                Some(Self::new(authority))
            }
        }
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
