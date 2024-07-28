use super::{
    super::parser::Parser,
    {host::Host, port::Port, userinfo::UserInfo},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityComponents {
    userinfo: Option<UserInfo>,
    host: Option<Host>,
    port: Option<Port>,
}

impl AuthorityComponents {
    pub(super) fn new(full: &str) -> Self {
        let mut parser = Parser::new(full.as_bytes());
        Self {
            userinfo: UserInfo::parse(&mut parser),
            host: Host::parse(&mut parser),
            port: Port::parse(&mut parser),
        }
    }
    pub fn userinfo(&self) -> Option<&UserInfo> {
        self.userinfo.as_ref()
    }
    pub fn userinfo_str(&self) -> Option<&str> {
        self.userinfo.as_deref()
    }
    pub fn host(&self) -> Option<&Host> {
        self.host.as_ref()
    }
    pub fn host_str(&self) -> Option<&str> {
        self.host.as_deref()
    }
    pub fn port(&self) -> Option<&Port> {
        self.port.as_ref()
    }
    pub fn port_str(&self) -> Option<&str> {
        self.port.as_deref()
    }
    pub fn port_u16(&self) -> Option<u16> {
        self.port_str().and_then(|s| s.parse::<u16>().ok())
    }
}
