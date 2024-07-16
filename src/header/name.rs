use super::ByteStr;

macro_rules! headers {
    (
        $(
            ($header:ident, $name:literal),
        )+
    ) => {
        $(
            pub const $header: HeaderName = HeaderName::from_static($name);
        )+
    };
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct HeaderName(ByteStr);

impl HeaderName {
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

impl From<&[u8]> for HeaderName {
    fn from(value: &[u8]) -> Self {
        unsafe { Self(ByteStr::from_utf8_unchecked(value)) }
    }
}

impl From<&str> for HeaderName {
    fn from(value: &str) -> Self {
        Self::from(value.as_bytes())
    }
}

impl From<String> for HeaderName {
    fn from(value: String) -> Self {
        Self::from(value.as_bytes())
    }
}

impl std::fmt::Display for HeaderName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

headers! {
    (ACCEPT, "accept"),
    (ACCEPT_CHARSET, "accept-charset"),
    (ACCEPT_ENCODING, "accept-encoding"),
    (ACCEPT_LANGUAGE, "accept-language"),
    (ACCEPT_RANGES, "accept-ranges"),
    (ACCESS_CONTROL_ALLOW_CREDENTIALS, "access-control-allow-credentials"),
    (ACCESS_CONTROL_ALLOW_HEADERS, "access-control-allow-headers"),
    (ACCESS_CONTROL_ALLOW_METHODS, "access-control-allow-methods"),
    (ACCESS_CONTROL_ALLOW_ORIGIN, "access-control-allow-origin"),
    (ACCESS_CONTROL_EXPOSE_HEADERS, "access-control-expose-headers"),
    (ACCESS_CONTROL_MAX_AGE, "access-control-max-age"),
    (ACCESS_CONTROL_REQUEST_HEADERS, "access-control-request-headers"),
    (ACCESS_CONTROL_REQUEST_METHOD, "access-control-request-method"),
    (AGE, "age"),
    (ALLOW, "allow"),
    (ALT_SVC, "alt-svc"),
    (AUTHORIZATION, "authorization"),
    (CACHE_CONTROL, "cache-control"),
    (CACHE_STATUS, "cache-status"),
    (CDN_CACHE_CONTROL, "cdn-cache-control"),
    (CONNECTION, "connection"),
    (CONTENT_DISPOSITION, "content-disposition"),
    (CONTENT_ENCODING, "content-encoding"),
    (CONTENT_LANGUAGE, "content-language"),
    (CONTENT_LENGTH, "content-length"),
    (CONTENT_LOCATION, "content-location"),
    (CONTENT_RANGE, "content-range"),
    (CONTENT_SECURITY_POLICY, "content-security-policy"),
    (CONTENT_SECURITY_POLICY_REPORT_ONLY, "content-security-policy-report-only"),
    (CONTENT_TYPE, "content-type"),
    (COOKIE, "cookie"),
    (DNT, "dnt"),
    (DATE, "date"),
    (ETAG, "etag"),
    (EXPECT, "expect"),
    (EXPIRES, "expires"),
    (FORWARDED, "forwarded"),
    (FROM, "from"),
    (HOST, "host"),
    (IF_MATCH, "if-match"),
    (IF_MODIFIED_SINCE, "if-modified-since"),
    (IF_NONE_MATCH, "if-none-match"),
    (IF_RANGE, "if-range"),
    (IF_UNMODIFIED_SINCE, "if-unmodified-since"),
    (LAST_MODIFIED, "last-modified"),
    (LINK, "link"),
    (LOCATION, "location"),
    (MAX_FORWARDS, "max-forwards"),
    (ORIGIN, "origin"),
    (PRAGMA, "pragma"),
    (PROXY_AUTHENTICATE, "proxy-authenticate"),
    (PROXY_AUTHORIZATION, "proxy-authorization"),
    (PUBLIC_KEY_PINS, "public-key-pins"),
    (PUBLIC_KEY_PINS_REPORT_ONLY, "public-key-pins-report-only"),
    (RANGE, "range"),
    (REFERER, "referer"),
    (REFERRER_POLICY, "referrer-policy"),
    (REFRESH, "refresh"),
    (RETRY_AFTER, "retry-after"),
    (SEC_WEBSOCKET_ACCEPT, "sec-websocket-accept"),
    (SEC_WEBSOCKET_EXTENSIONS, "sec-websocket-extensions"),
    (SEC_WEBSOCKET_KEY, "sec-websocket-key"),
    (SEC_WEBSOCKET_PROTOCOL, "sec-websocket-protocol"),
    (SEC_WEBSOCKET_VERSION, "sec-websocket-version"),
    (SERVER, "server"),
    (SET_COOKIE, "set-cookie"),
    (STRICT_TRANSPORT_SECURITY, "strict-transport-security"),
    (TE, "te"),
    (TRAILER, "trailer"),
    (TRANSFER_ENCODING, "transfer-encoding"),
    (USER_AGENT, "user-agent"),
    (UPGRADE, "upgrade"),
    (UPGRADE_INSECURE_REQUESTS, "upgrade-insecure-requests"),
    (VARY, "vary"),
    (VIA, "via"),
    (WARNING, "warning"),
    (WWW_AUTHENTICATE, "www-authenticate"),
    (X_CONTENT_TYPE_OPTIONS, "x-content-type-options"),
    (X_DNS_PREFETCH_CONTROL, "x-dns-prefetch-control"),
    (X_FRAME_OPTIONS, "x-frame-options"),
    (X_XSS_PROTECTION, "x-xss-protection"),
}
