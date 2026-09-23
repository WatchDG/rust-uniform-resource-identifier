use bytes::Bytes;

use crate::grammar::validate_userinfo;
use crate::UriError;

#[derive(Debug, Clone, PartialEq)]
pub struct Userinfo {
    pub origin: Bytes,
}

impl Userinfo {
    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.origin.clone()
    }

    #[inline]
    pub fn from_bytes(input: Bytes) -> Result<Self, UriError> {
        validate_userinfo(&input)?;
        Ok(Self { origin: input })
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Result<Self, UriError> {
        Self::from_bytes(Bytes::copy_from_slice(input))
    }
}
