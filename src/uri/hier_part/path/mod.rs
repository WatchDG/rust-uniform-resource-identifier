use bytes::Bytes;

use crate::grammar::validate_path_value;
use crate::UriError;

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub origin: Bytes,
}

impl Path {
    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.origin.clone()
    }

    #[inline]
    pub fn from_bytes(input: Bytes) -> Result<Self, UriError> {
        validate_path_value(&input)?;
        Ok(Self { origin: input })
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Result<Self, UriError> {
        Self::from_bytes(Bytes::copy_from_slice(input))
    }
}
