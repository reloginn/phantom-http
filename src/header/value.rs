use crate::header::ByteStr;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct HeaderValue(ByteStr);

impl HeaderValue {
    pub const fn from_static(s: &'static str) -> Self {
        Self(ByteStr::from_static(s))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl From<&[u8]> for HeaderValue {
    fn from(value: &[u8]) -> Self {
        unsafe { Self(ByteStr::from_utf8_unchecked(value)) }
    }
}

impl From<&str> for HeaderValue {
    fn from(value: &str) -> Self {
        Self::from(value.as_bytes())
    }
}

impl From<String> for HeaderValue {
    fn from(value: String) -> Self {
        Self::from(value.as_bytes())
    }
}

impl std::fmt::Display for HeaderValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
