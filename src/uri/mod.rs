use bytes::{BufMut, Bytes, BytesMut};

use crate::grammar::{scheme_colon, validate_fragment, validate_query, validate_scheme};
use crate::UriError;

mod fragment;
mod hier_part;
mod query;
mod scheme;

pub use fragment::Fragment;
pub use hier_part::{Authority, HierPart, HierPartBuilder, Host, Path, Port, Userinfo};
pub use query::{encode_pairs, Query, QueryPairs};
pub use scheme::Scheme;

use hier_part::{hier_wire_len, parse_hier, validate_hier, write_hier};

#[derive(Debug, Clone, PartialEq)]
pub struct Uri {
    pub origin: Bytes,
    pub scheme: Option<Scheme>,
    pub hier_part: HierPart,
    pub query: Option<Query>,
    pub fragment: Option<Fragment>,
}

impl Uri {
    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.origin.clone()
    }

    pub fn parse(input: Bytes) -> Result<Self, UriError> {
        let mut cursor = 0;
        let end = input.len();
        let builder = UriBuilder::parse(&input, &mut cursor, end)?;
        let hier_part = match builder.hier_part {
            Some(hier_part) => hier_part,
            None => return Err(UriError::InvalidUri),
        };
        Ok(Self {
            origin: input,
            scheme: builder.scheme,
            hier_part,
            query: builder.query,
            fragment: builder.fragment,
        })
    }

    #[inline]
    pub fn parse_slice(input: &[u8]) -> Result<Self, UriError> {
        Self::parse(Bytes::copy_from_slice(input))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UriBuilder {
    pub scheme: Option<Scheme>,
    pub hier_part: Option<HierPart>,
    pub query: Option<Query>,
    pub fragment: Option<Fragment>,
}

impl UriBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            scheme: None,
            hier_part: None,
            query: None,
            fragment: None,
        }
    }

    pub fn scheme(&mut self, scheme: Scheme) -> &mut Self {
        self.scheme = Some(scheme);
        self
    }

    pub fn hier_part(&mut self, hier_part: HierPart) -> &mut Self {
        self.hier_part = Some(hier_part);
        self
    }

    pub fn query(&mut self, query: Query) -> &mut Self {
        self.query = Some(query);
        self
    }

    pub fn fragment(&mut self, fragment: Fragment) -> &mut Self {
        self.fragment = Some(fragment);
        self
    }

    pub fn parse(input: &Bytes, start: &mut usize, end: usize) -> Result<Self, UriError> {
        if *start > end || end > input.len() {
            return Err(UriError::InvalidUri);
        }
        let base = *start;
        let scheme = if let Some(colon) = scheme_colon(&input[base..end]) {
            let colon_at = base + colon;
            let scheme = Scheme {
                origin: input.slice(base..colon_at),
            };
            *start = colon_at + 1;
            Some(scheme)
        } else {
            None
        };
        let hier_part = parse_hier(input, start, end, scheme.is_some())?;
        let query = if *start < end && input[*start] == b'?' {
            Some(Query::parse(input, start, end)?)
        } else {
            None
        };
        let fragment = if *start < end && input[*start] == b'#' {
            Some(Fragment::parse(input, start, end)?)
        } else {
            None
        };
        if *start != end {
            return Err(UriError::InvalidUri);
        }
        Ok(Self {
            scheme,
            hier_part: Some(hier_part),
            query,
            fragment,
        })
    }

    pub fn build(&self) -> Result<Uri, UriError> {
        if let Some(scheme) = &self.scheme {
            validate_scheme(&scheme.origin)?;
        }
        if let Some(hier_part) = &self.hier_part {
            validate_hier(hier_part, self.scheme.is_some())?;
        }
        if let Some(query) = &self.query {
            validate_query(&query.origin)?;
        }
        if let Some(fragment) = &self.fragment {
            validate_fragment(&fragment.origin)?;
        }
        let len = uri_len(self);
        let mut out = BytesMut::with_capacity(len);
        write_uri(self, &mut out);
        debug_assert_eq!(out.len(), len);
        Uri::parse(out.freeze())
    }
}

fn uri_len(builder: &UriBuilder) -> usize {
    let mut len = 0;
    if let Some(scheme) = &builder.scheme {
        len += scheme.origin.len() + 1;
    }
    if let Some(hier_part) = &builder.hier_part {
        len += hier_wire_len(hier_part);
    }
    if let Some(query) = &builder.query {
        len += 1 + query.origin.len();
    }
    if let Some(fragment) = &builder.fragment {
        len += 1 + fragment.origin.len();
    }
    len
}

fn write_uri(builder: &UriBuilder, out: &mut BytesMut) {
    if let Some(scheme) = &builder.scheme {
        out.put_slice(&scheme.origin);
        out.put_u8(b':');
    }
    if let Some(hier_part) = &builder.hier_part {
        write_hier(hier_part, out);
    }
    if let Some(query) = &builder.query {
        out.put_u8(b'?');
        out.put_slice(&query.origin);
    }
    if let Some(fragment) = &builder.fragment {
        out.put_u8(b'#');
        out.put_slice(&fragment.origin);
    }
}
