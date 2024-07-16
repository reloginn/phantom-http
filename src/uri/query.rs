/// query       = *( pchar / "/" / "?" )
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query(std::sync::Arc<str>);

impl Query {
    pub(crate) fn new(value: &str) -> Self {
        Self(std::sync::Arc::from(value))
    }
    pub fn parse(uri: &str) -> Option<Self> {
        let query = uri.split_once('?').map(|(_, query_and_maybe_fragment)| {
            query_and_maybe_fragment
                .split_once('#')
                .map(|(query, _)| query)
                .unwrap_or(query_and_maybe_fragment)
        })?;
        Some(Self::new(query))
    }
}

impl std::ops::Deref for Query {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
