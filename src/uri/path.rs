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
pub struct Path(Arc<str>);

impl Path {
    pub(crate) fn new(value: &str) -> Self {
        Self(std::sync::Arc::from(value))
    }
    pub fn parse(uri: &str) -> Option<Self> {
        let path = uri
            .split_once("//")
            .and_then(|(_, authority_and_maybe_path)| {
                let len = authority_and_maybe_path.len();
                let mut start = 0;
                let mut end = 0;
                for (index, &byte) in authority_and_maybe_path.as_bytes().iter().enumerate() {
                    if byte == b'/' && start == 0 {
                        start = index;
                    } else if byte == b'/' && index == len {
                        return None;
                    }
                    if byte == b'?' || byte == b'#' && start != 0 && end == 0 {
                        end = index;
                    } else if byte == b'?' || byte == b'#' && start == 0 {
                        return None;
                    }
                }
                match (start, end) {
                    (0, 0) => None,
                    (start, 0) if start != 0 => Some(&authority_and_maybe_path[start..]),
                    (start, end) => Some(&authority_and_maybe_path[start..end]),
                }
            })
            .or(uri.split_once(':').and_then(|(_, maybe_path)| {
                if !maybe_path.starts_with("//") {
                    // if `maybe_path` does not start with `//`, then it is indeed path
                    Some(maybe_path)
                } else {
                    // ...otherwise it is Authority, in which case `path` must be None
                    None
                }
            }));
        path.map(Self::new)
    }
}

impl std::ops::Deref for Path {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
