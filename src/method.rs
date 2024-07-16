#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum Method {
    #[default]
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
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
            _ => panic!("unknown HTTP method"),
        }
    }
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
