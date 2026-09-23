use bytes::Bytes;

use crate::grammar::validate_fragment;
use crate::UriError;

#[derive(Debug, Clone, PartialEq)]
pub struct Fragment {
    pub origin: Bytes,
}

impl Fragment {
    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.origin.clone()
    }

    #[inline]
    pub fn from_bytes(input: Bytes) -> Result<Self, UriError> {
        validate_fragment(&input)?;
        Ok(Self { origin: input })
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Result<Self, UriError> {
        Self::from_bytes(Bytes::copy_from_slice(input))
    }

    #[inline]
    pub fn parse(input: &Bytes, start: &mut usize, end: usize) -> Result<Self, UriError> {
        if *start >= end || end > input.len() || input[*start] != b'#' {
            return Err(UriError::InvalidFragment);
        }
        *start += 1;
        let from = *start;
        validate_fragment(&input[from..end])?;
        let value = Self {
            origin: input.slice(from..end),
        };
        *start = end;
        Ok(value)
    }
}
