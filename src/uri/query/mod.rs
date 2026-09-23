use bytes::{BufMut, Bytes, BytesMut};

use crate::charset::EncodeSet;
use crate::grammar::validate_query;
use crate::pct::{encode_into, encoded_len};
use crate::UriError;

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub origin: Bytes,
}

impl Query {
    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.origin.clone()
    }

    #[inline]
    pub fn from_bytes(input: Bytes) -> Result<Self, UriError> {
        validate_query(&input)?;
        Ok(Self { origin: input })
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Result<Self, UriError> {
        Self::from_bytes(Bytes::copy_from_slice(input))
    }

    pub fn parse(input: &Bytes, start: &mut usize, end: usize) -> Result<Self, UriError> {
        if *start >= end || end > input.len() || input[*start] != b'?' {
            return Err(UriError::InvalidQuery);
        }
        *start += 1;
        let from = *start;
        while *start < end && input[*start] != b'#' {
            *start += 1;
        }
        validate_query(&input[from..*start])?;
        Ok(Self {
            origin: input.slice(from..*start),
        })
    }

    pub fn pairs(&self) -> QueryPairs<'_> {
        QueryPairs {
            origin: &self.origin,
            pos: 0,
        }
    }
}

#[derive(Debug)]
pub struct QueryPairs<'a> {
    origin: &'a Bytes,
    pos: usize,
}

impl<'a> Iterator for QueryPairs<'a> {
    type Item = (Bytes, Option<Bytes>);

    fn next(&mut self) -> Option<Self::Item> {
        let data = self.origin.as_ref();
        if data.is_empty() || self.pos > data.len() {
            return None;
        }
        let rest = &data[self.pos..];
        let amp = rest.iter().position(|&byte| byte == b'&');
        let rel = amp.unwrap_or(rest.len());
        let start = self.pos;
        let end = self.pos + rel;
        if amp.is_some() {
            self.pos = end + 1;
        } else {
            self.pos = data.len() + 1;
        }
        let piece = &data[start..end];
        if let Some(eq) = piece.iter().position(|&byte| byte == b'=') {
            let key = self.origin.slice(start..start + eq);
            let value = self.origin.slice(start + eq + 1..end);
            Some((key, Some(value)))
        } else {
            Some((self.origin.slice(start..end), None))
        }
    }
}

pub fn encode_pairs<I, K, V>(pairs: I) -> Bytes
where
    I: IntoIterator<Item = (K, Option<V>)> + Clone,
    K: AsRef<[u8]>,
    V: AsRef<[u8]>,
{
    let mut len = 0;
    let mut count = 0usize;
    for (key, value) in pairs.clone() {
        if count > 0 {
            len += 1;
        }
        len += encoded_len(key.as_ref(), EncodeSet::QUERY_PAIR);
        if let Some(value) = value {
            len += 1 + encoded_len(value.as_ref(), EncodeSet::QUERY_PAIR);
        }
        count += 1;
    }
    let mut out = BytesMut::with_capacity(len);
    let mut first = true;
    for (key, value) in pairs {
        if !first {
            out.put_u8(b'&');
        }
        first = false;
        encode_into(key.as_ref(), EncodeSet::QUERY_PAIR, &mut out);
        if let Some(value) = value {
            out.put_u8(b'=');
            encode_into(value.as_ref(), EncodeSet::QUERY_PAIR, &mut out);
        }
    }
    debug_assert_eq!(out.len(), len);
    out.freeze()
}
