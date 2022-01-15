use bytes::Bytes;

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
    pub fn from_bytes(input: Bytes) -> Self {
        Self { origin: input }
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Self {
        let bytes = Bytes::copy_from_slice(input);
        Self::from_bytes(bytes)
    }

    pub fn parse(input: &[u8], start: &mut usize, end: &usize) -> Result<Self, UriError> {
        let mut index = *start;
        while index < *end && input[index] != 0x3a {
            index += 1;
        }
        if input[index] != 0x3a {
            return Err(UriError::InvalidScheme);
        }
        index += 1;
        let value = Self::from_slice(&input[*start..index]);
        *start = index;
        Ok(value)
    }
}

#[cfg(test)]
mod tests_scheme {
    use crate::Scheme;
    use bytes::Bytes;

    #[test]
    fn test_bytes() {
        let scheme = Scheme::from_bytes(Bytes::from_static(b"foo:"));
        assert_eq!(scheme.bytes(), Bytes::from_static(b"foo:"));
    }

    #[test]
    fn test_from_bytes() {
        let scheme = Scheme::from_bytes(Bytes::from_static(b"foo:"));
        assert_eq!(scheme.origin, Bytes::from_static(b"foo:"));
    }

    #[test]
    fn test_from_slice() {
        let scheme = Scheme::from_slice(b"foo:");
        assert_eq!(scheme.origin, Bytes::from_static(b"foo:"));
    }

    #[test]
    fn test_parse() {
        let string = "foo://example.com:8042/over/there?name=ferret#nose";
        let mut cursor = 0;
        let scheme = Scheme::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap();
        assert_eq!(scheme.origin, Bytes::from_static(b"foo:"));
        assert_eq!(cursor, 4);
    }
}
