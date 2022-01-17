use bytes::Bytes;

use crate::UriError;

mod host;

pub use host::Host;

#[derive(Debug, Clone, PartialEq)]
pub struct Authority {
    pub origin: Bytes,
}

impl Authority {
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

    pub fn parse(input: &[u8], start: &mut usize, end: &usize) -> Result<Self, UriError> {
        let mut index = *start;
        while index < *end && input[index] != 0x2f {
            index += 1;
        }
        let value = Self::from_slice(&input[*start..index]);
        *start = index;
        Ok(value)
    }
}

#[cfg(test)]
pub mod tests_authority {
    use crate::Authority;
    use bytes::Bytes;

    #[test]
    fn test_bytes() {
        let authority = Authority::from_bytes(Bytes::from_static(b"example.com"));
        assert_eq!(authority.bytes(), Bytes::from_static(b"example.com"))
    }

    #[test]
    fn test_from_bytes() {
        let authority = Authority::from_bytes(Bytes::from_static(b"example.com"));
        assert_eq!(authority.origin, Bytes::from_static(b"example.com"))
    }

    #[test]
    fn test_from_slice() {
        let authority = Authority::from_slice(b"example.com");
        assert_eq!(authority.origin, Bytes::from_static(b"example.com"))
    }

    #[test]
    fn test_parse() {
        let string = "foo://example.com:8042/over/there?name=ferret#nose";
        let mut cursor = 6;
        let authority = Authority::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap();
        assert_eq!(authority.bytes(), Bytes::from_static(b"example.com:8042"));
        assert_eq!(cursor, 22);
    }
}
