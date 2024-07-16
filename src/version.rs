#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum Version {
    HTTP09,
    HTTP10,
    HTTP11,
    HTTP2,
    #[default]
    HTTP3,
}

impl From<&[u8]> for Version {
    fn from(value: &[u8]) -> Self {
        use Version::*;
        match value {
            b"HTTP/0.9" => HTTP09,
            b"HTTP/1.0" => HTTP10,
            b"HTTP/1.1" => HTTP11,
            b"HTTP/2.0" => HTTP2,
            b"HTTP/3.0" => HTTP3,
            _ => panic!("unknown HTTP version"),
        }
    }
}

impl From<String> for Version {
    fn from(value: String) -> Self {
        Self::from(value.as_bytes())
    }
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        Self::from(value.as_bytes())
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Version::*;
        let s = match self {
            HTTP09 => "HTTP/0.9",
            HTTP10 => "HTTP/1.0",
            HTTP11 => "HTTP/1.1",
            HTTP2 => "HTTP/2.0",
            HTTP3 => "HTTP/3.0",
        };
        f.write_str(s)
    }
}
