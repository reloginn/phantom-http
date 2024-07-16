/// fragment    = *( pchar / "/" / "?" )
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fragment(std::sync::Arc<str>);

impl Fragment {
    pub(crate) fn new(value: &str) -> Self {
        Self(std::sync::Arc::from(value))
    }
    pub fn parse(uri: &str) -> Option<Self> {
        let fragment = uri.split_once('#').map(|(_, fragment)| fragment)?;
        Some(Self::new(fragment))
    }
}

impl std::ops::Deref for Fragment {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
