/// authority = [ userinfo «@» ] host [ «:» port ]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authority(std::sync::Arc<str>);

impl Authority {
    pub(crate) fn new(value: &str) -> Self {
        Self(std::sync::Arc::from(value))
    }
    pub fn parse(uri: &str) -> Option<Self> {
        let authority = uri.split_once("//").map(|(_, uri_part)| {
            uri_part
                .split_once(|ch| ch == '/' || ch == '?' || ch == '#')
                .map(|(authority, _)| authority)
                .unwrap_or(uri_part)
        })?;
        Some(Self(std::sync::Arc::from(authority)))
    }
}

impl std::ops::Deref for Authority {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Authority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
