use std::num::NonZeroU16;

macro_rules! status_codes {
    (
        $(
            ($num:expr, $status_code:ident, $phrase:expr),
        )+
    ) => {
        impl StatusCode {
        $(
            pub const $status_code: StatusCode = StatusCode(unsafe { NonZeroU16::new_unchecked($num) });
        )+
        }

        fn to_str(num: u16) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    };
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct StatusCode(NonZeroU16);

impl StatusCode {
    pub fn from_u16(src: u16) -> Result<Self, InvalidStatusCode> {
        if !(100..1000).contains(&src) {
            return Err(InvalidStatusCode);
        }
        NonZeroU16::new(src).map(Self).ok_or(InvalidStatusCode)
    }
    pub fn as_u16(&self) -> u16 {
        self.0.get()
    }
    pub fn to_str(&self) -> Option<&'static str> {
        to_str(self.as_u16())
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        Self::from_u16(200).unwrap()
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            u16::from(*self),
            self.to_str().unwrap_or("<unknown>")
        )
    }
}

impl TryFrom<u16> for StatusCode {
    type Error = InvalidStatusCode;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value)
    }
}

impl From<StatusCode> for u16 {
    fn from(value: StatusCode) -> Self {
        value.as_u16()
    }
}

status_codes! {
    (100, CONTINUE, "Continue"),
    (101, SWITCHING_PROTOCOLS, "Switching Protocols"),
    (102, PROCESSING, "Processing"),

    (200, OK, "OK"),
    (201, CREATED, "Created"),
    (202, ACCEPTED, "Accepted"),
    (203, NON_AUTHORITATIVE_INFORMATION, "Non Authoritative Information"),
    (204, NO_CONTENT, "No Content"),
    (205, RESET_CONTENT, "Reset Content"),
    (206, PARTIAL_CONTENT, "Partial Content"),
    (207, MULTI_STATUS, "Multi-Status"),
    (208, ALREADY_REPORTED, "Already Reported"),
    (226, IM_USED, "IM Used"),

    (300, MULTIPLE_CHOICES, "Multiple Choices"),
    (301, MOVED_PERMANENTLY, "Moved Permanently"),
    (302, FOUND, "Found"),
    (303, SEE_OTHER, "See Other"),
    (304, NOT_MODIFIED, "Not Modified"),
    (305, USE_PROXY, "Use Proxy"),
    (307, TEMPORARY_REDIRECT, "Temporary Redirect"),
    (308, PERMANENT_REDIRECT, "Permanent Redirect"),

    (400, BAD_REQUEST, "Bad Request"),
    (401, UNAUTHORIZED, "Unauthorized"),
    (402, PAYMENT_REQUIRED, "Payment Required"),
    (403, FORBIDDEN, "Forbidden"),
    (404, NOT_FOUND, "Not Found"),
    (405, METHOD_NOT_ALLOWED, "Method Not Allowed"),
    (406, NOT_ACCEPTABLE, "Not Acceptable"),
    (407, PROXY_AUTHENTICATION_REQUIRED, "Proxy Authentication Required"),
    (408, REQUEST_TIMEOUT, "Request Timeout"),
    (409, CONFLICT, "Conflict"),
    (410, GONE, "Gone"),
    (411, LENGTH_REQUIRED, "Length Required"),
    (412, PRECONDITION_FAILED, "Precondition Failed"),
    (413, PAYLOAD_TOO_LARGE, "Payload Too Large"),
    (414, URI_TOO_LONG, "URI Too Long"),
    (415, UNSUPPORTED_MEDIA_TYPE, "Unsupported Media Type"),
    (416, RANGE_NOT_SATISFIABLE, "Range Not Satisfiable"),
    (417, EXPECTATION_FAILED, "Expectation Failed"),
    (418, IM_A_TEAPOT, "I'm a teapot"),
    (421, MISDIRECTED_REQUEST, "Misdirected Request"),
    (422, UNPROCESSABLE_ENTITY, "Unprocessable Entity"),
    (423, LOCKED, "Locked"),
    (424, FAILED_DEPENDENCY, "Failed Dependency"),
    (426, UPGRADE_REQUIRED, "Upgrade Required"),
    (428, PRECONDITION_REQUIRED, "Precondition Required"),
    (429, TOO_MANY_REQUESTS, "Too Many Requests"),
    (431, REQUEST_HEADER_FIELDS_TOO_LARGE, "Request Header Fields Too Large"),
    (451, UNAVAILABLE_FOR_LEGAL_REASONS, "Unavailable For Legal Reasons"),

    (500, INTERNAL_SERVER_ERROR, "Internal Server Error"),
    (501, NOT_IMPLEMENTED, "Not Implemented"),
    (502, BAD_GATEWAY, "Bad Gateway"),
    (503, SERVICE_UNAVAILABLE, "Service Unavailable"),
    (504, GATEWAY_TIMEOUT, "Gateway Timeout"),
    (505, HTTP_VERSION_NOT_SUPPORTED, "HTTP Version Not Supported"),
    (506, VARIANT_ALSO_NEGOTIATES, "Variant Also Negotiates"),
    (507, INSUFFICIENT_STORAGE, "Insufficient Storage"),
    (510, NOT_EXTENDED, "Not Extended"),
    (511, NETWORK_AUTHENTICATION_REQUIRED, "Network Authentication Required"),
}

#[derive(Debug)]
pub struct InvalidStatusCode;

impl std::fmt::Display for InvalidStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid status code")
    }
}

impl std::error::Error for InvalidStatusCode {}
