/// scheme = ALPHA *( ALPHA / DIGIT / «+» / «-» / «.» )
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Scheme(std::sync::Arc<str>);

impl Scheme {
    pub(crate) fn new(value: &str) -> Self {
        Self(std::sync::Arc::from(value))
    }
    pub fn parse(uri: &str) -> Option<Self> {
        let scheme = uri.split_once(':').map(|(scheme, _)| scheme)?;
        Some(Self::new(scheme))
    }
}

impl std::ops::Deref for Scheme {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self)
    }
}
