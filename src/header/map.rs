use super::{name::HeaderName, value::HeaderValue};
use std::collections::HashMap;

const DEFAULT_MAP_CAPACITY: usize = 128;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct HeaderMap(HashMap<HeaderName, HeaderValue>);

impl HeaderMap {
    pub fn new() -> Self {
        Self(HashMap::with_capacity(DEFAULT_MAP_CAPACITY))
    }
    pub fn insert(&mut self, name: HeaderName, value: HeaderValue) -> Option<HeaderValue> {
        self.0.insert(name, value)
    }
    pub fn get(&self, name: &HeaderName) -> Option<&HeaderValue> {
        self.0.get(name)
    }
    pub fn remove(&mut self, name: &HeaderName) -> Option<HeaderValue> {
        self.0.remove(name)
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<HeaderName, HeaderValue> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<HeaderName, HeaderValue> {
        self.0.iter_mut()
    }
    pub fn keys(&self) -> std::collections::hash_map::Keys<HeaderName, HeaderValue> {
        self.0.keys()
    }
    pub fn into_keys(self) -> std::collections::hash_map::IntoKeys<HeaderName, HeaderValue> {
        self.0.into_keys()
    }
    pub fn values(&self) -> std::collections::hash_map::Values<HeaderName, HeaderValue> {
        self.0.values()
    }
    pub fn into_values(self) -> std::collections::hash_map::IntoValues<HeaderName, HeaderValue> {
        self.0.into_values()
    }
}

impl IntoIterator for HeaderMap {
    type Item = (HeaderName, HeaderValue);
    type IntoIter = std::collections::hash_map::IntoIter<HeaderName, HeaderValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
