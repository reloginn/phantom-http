use super::{
    macros::{get_unchecked, to_compact},
    parser::{Parser, State},
};
use compact_str::CompactString;

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
pub struct Path(CompactString);

impl Path {
    pub(crate) fn new(value: impl AsRef<[u8]>) -> Self {
        let compact = to_compact!(value);
        Self(compact)
    }
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        let from = parser.position();
        while parser.state() != State::Eof {
            let byte = parser.get_byte();
            if byte == b'?' || byte == b'#' {
                return Some(get_unchecked!(parser, from, parser.position()));
            }
            if byte == b':' {
                return Some(get_unchecked!(parser, parser.position()));
            }
            parser.increment()
        }
        if from == parser.eof() {
            None
        } else {
            Some(get_unchecked!(parser, from))
        }
    }
}

impl std::ops::Deref for Path {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
