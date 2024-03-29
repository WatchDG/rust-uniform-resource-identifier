use bytes::{BufMut, Bytes, BytesMut};
use std::panic;

mod fragment;
mod hier_part;
mod query;
mod scheme;

pub use fragment::Fragment;
pub use hier_part::{Authority, HierPart, Path};
pub use query::Query;
pub use scheme::Scheme;

use crate::UriError;

#[derive(Debug, Clone, PartialEq)]
pub struct Uri {
    pub origin: Bytes,
}

impl Uri {
    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.origin.clone()
    }

    #[inline]
    pub fn from_bytes(input: Bytes) -> Self {
        Self { origin: input }
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Self {
        let bytes = Bytes::copy_from_slice(input);
        Self { origin: bytes }
    }
}

#[cfg(test)]
mod tests_uri {
    use crate::Uri;
    use bytes::Bytes;

    #[test]
    fn test_bytes() {
        let uri = Uri::from_bytes(Bytes::from_static(
            b"foo://example.com:8042/over/there?name=ferret#nose",
        ));
        assert_eq!(
            uri.bytes(),
            Bytes::from_static(b"foo://example.com:8042/over/there?name=ferret#nose")
        );
    }

    #[test]
    fn test_from_bytes() {
        let uri = Uri::from_bytes(Bytes::from_static(
            b"foo://example.com:8042/over/there?name=ferret#nose",
        ));
        assert_eq!(
            uri.origin,
            Bytes::from_static(b"foo://example.com:8042/over/there?name=ferret#nose")
        );
    }

    #[test]
    fn test_from_slice() {
        let uri = Uri::from_slice(b"foo://example.com:8042/over/there?name=ferret#nose");
        assert_eq!(
            uri.origin,
            Bytes::from_static(b"foo://example.com:8042/over/there?name=ferret#nose")
        );
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

    pub fn scheme(&mut self, scheme: Scheme) -> &Self {
        self.scheme = Some(scheme);
        self
    }

    pub fn hier_part(&mut self, hier_part: HierPart) -> &Self {
        self.hier_part = Some(hier_part);
        self
    }

    pub fn query(&mut self, query: Query) -> &Self {
        self.query = Some(query);
        self
    }

    pub fn fragment(&mut self, fragment: Fragment) -> &Self {
        self.fragment = Some(fragment);
        self
    }

    pub fn parse(input: &[u8], start: &mut usize, end: &usize) -> Result<Self, UriError> {
        let mut index = *start;
        let mut uri_builder = Self::new();
        uri_builder.scheme(Scheme::parse(input, &mut index, end)?);
        uri_builder.hier_part(HierPart::parse(input, &mut index, end)?);
        while index < *end {
            match input[index] {
                0x3f => {
                    uri_builder.query(Query::parse(input, &mut index, end)?);
                }
                0x23 => {
                    uri_builder.fragment(Fragment::parse(input, &mut index, end)?);
                }
                _ => {
                    break;
                }
            }
        }
        if index != *end {
            return Err(UriError::InvalidUri);
        }
        *start = index;
        Ok(uri_builder)
    }

    pub fn build(&self) -> Result<Uri, UriError> {
        let mut bytes = BytesMut::new();
        match &self.scheme {
            Some(scheme) => {
                bytes.put(scheme.bytes());
            }
            None => {
                panic!("")
            }
        }
        Ok(Uri::from_bytes(bytes.freeze()))
    }
}

#[cfg(test)]
mod tests_uri_builder {
    use crate::{Fragment, HierPart, Query, Scheme, UriBuilder};
    use bytes::Bytes;

    #[test]
    fn test_parse() {
        let string = "foo://example.com:8042/over/there?name=ferret#nose";
        let mut cursor = 0;
        let uri_builder = UriBuilder::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap();

        let mut reference_uri_builder = UriBuilder::new();
        reference_uri_builder.scheme(Scheme::from_slice(b"foo:"));
        reference_uri_builder.hier_part(HierPart::from_slice(b"//example.com:8042/over/there"));
        reference_uri_builder.query(Query::from_slice(b"?name=ferret"));
        reference_uri_builder.fragment(Fragment::from_slice(b"#nose"));

        assert_eq!(uri_builder, reference_uri_builder);
        assert_eq!(cursor, 50);
    }

    #[test]
    fn test_build() {
        let uri = UriBuilder::new()
            .scheme(Scheme::from_slice(b"http:"))
            .build()
            .unwrap();
        assert_eq!(uri.bytes(), Bytes::from_static(b"http:"));
    }
}
