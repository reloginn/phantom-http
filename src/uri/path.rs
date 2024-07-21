use super::parser::{Parser, State};
use std::sync::Arc;

/// path          = path-abempty    ; begins with "/" or is empty
///               / path-absolute   ; begins with "/" but not "//"
///               / path-noscheme   ; begins with a non-colon segment
///               / path-rootless   ; begins with a segment
///               / path-empty      ; zero characters
///
/// path-abempty  = *( "/" segment )
/// path-absolute = "/" [ segment-nz *( "/" segment ) ]
/// path-noscheme = segment-nz-nc *( "/" segment )
/// path-rootless = segment-nz *( "/" segment )
/// path-empty    = 0<pchar>
/// segment       = *pchar
/// segment-nz    = 1*pchar
/// segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
///               ; non-zero-length segment without any colon ":"
///
/// pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path(Arc<[u8]>);

impl Path {
    pub(crate) fn new(value: impl AsRef<[u8]>) -> Self {
        Self(Arc::from(value.as_ref()))
    }
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        let start = parser.position();
        while parser.state() != State::Eof {
            let &byte = unsafe { parser.get_unchecked(parser.position()) };
            if byte == b'?' || byte == b'#' {
                let path = unsafe { parser.get_unchecked(start..parser.position()) };
                return Some(Self::new(path));
            }
            if byte == b':' {
                let path = unsafe { parser.get_unchecked(start..) };
                return Some(Self::new(path));
            }
            parser.increment()
        }
        // FIXME: if there is no `?` or `#` but there is Path, it throws `None`
        None
    }
}

impl std::ops::Deref for Path {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
