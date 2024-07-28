#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl Default for Method {
    fn default() -> Self {
        Self::GET
    }
}

impl From<&[u8]> for Method {
    fn from(value: &[u8]) -> Self {
        use Method::*;
        match value {
            b"GET" => GET,
            b"HEAD" => HEAD,
            b"POST" => POST,
            b"PUT" => PUT,
            b"DELETE" => DELETE,
            b"CONNECT" => CONNECT,
            b"OPTIONS" => OPTIONS,
            b"TRACE" => TRACE,
            other => method_from_bytes_fail(other),
        }
    }
}
#[inline(never)]
#[cold]
#[track_caller]
fn method_from_bytes_fail(method: &[u8]) -> ! {
    panic!("unknown http method: {method:?}")
}

impl From<String> for Method {
    fn from(value: String) -> Self {
        Self::from(value.as_bytes())
    }
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        Self::from(value.as_bytes())
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Method::*;
        let s = match self {
            GET => "GET",
            HEAD => "HEAD",
            POST => "POST",
            PUT => "PUT",
            DELETE => "DELETE",
            CONNECT => "CONNECT",
            OPTIONS => "OPTIONS",
            TRACE => "TRACE",
        };
        f.write_str(s)
    }
}
