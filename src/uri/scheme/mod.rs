use bytes::Bytes;

use crate::grammar::{scheme_colon, validate_scheme};
use crate::UriError;

#[derive(Debug, Clone, PartialEq)]
pub struct Scheme {
    pub origin: Bytes,
}

impl Scheme {
    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.origin.clone()
    }

    #[inline]
    pub fn from_bytes(input: Bytes) -> Result<Self, UriError> {
        validate_scheme(&input)?;
        Ok(Self { origin: input })
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Result<Self, UriError> {
        Self::from_bytes(Bytes::copy_from_slice(input))
    }

    pub fn parse(input: &Bytes, start: &mut usize, end: usize) -> Result<Self, UriError> {
        if *start > end || end > input.len() {
            return Err(UriError::InvalidScheme);
        }
        let Some(colon) = scheme_colon(&input[*start..end]) else {
            return Err(UriError::InvalidScheme);
        };
        let from = *start;
        let colon_at = *start + colon;
        let scheme = Self {
            origin: input.slice(from..colon_at),
        };
        *start = colon_at + 1;
        Ok(scheme)
    }
}
