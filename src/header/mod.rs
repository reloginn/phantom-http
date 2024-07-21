use bytes::Bytes;

pub mod map;
pub mod name;
pub mod value;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ByteStr(Bytes);

impl Default for ByteStr {
    fn default() -> Self {
        Self::empty()
    }
}

impl ByteStr {
    pub const fn empty() -> ByteStr {
        Self(Bytes::new())
    }
    pub const fn from_static(value: &'static str) -> ByteStr {
        Self(Bytes::from_static(value.as_bytes()))
    }
    /// ## Safety
    /// `bytes` must contain valid UTF-8. In a release build it is undefined
    /// behaviour to call this with `bytes` that is not valid UTF-8.
    pub unsafe fn from_utf8_unchecked(bytes: &[u8]) -> Self {
        if cfg!(debug_assertions) {
            match std::str::from_utf8(bytes) {
                Ok(_) => {}
                Err(err) => panic!(
                    "[`ByteStr::from_utf8_unchecked()`] with invalid bytes: {}",
                    err
                ),
            }
        }
        Self(Bytes::copy_from_slice(bytes))
    }
}

impl std::ops::Deref for ByteStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}
