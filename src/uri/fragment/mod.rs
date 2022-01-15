use bytes::Bytes;

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
        while index < *end {
            index += 1;
        }
        let value = Self::from_slice(&input[*start..index]);
        *start = index;
        Ok(value)
    }
}

#[cfg(test)]
mod tests_fragment {
    use crate::Fragment;
    use bytes::Bytes;

    #[test]
    fn test_bytes() {
        let fragment = Fragment::from_bytes(Bytes::from_static(b"#nose"));
        assert_eq!(fragment.bytes(), Bytes::from_static(b"#nose"));
    }

    #[test]
    fn test_from_bytes() {
        let fragment = Fragment::from_bytes(Bytes::from_static(b"#nose"));
        assert_eq!(fragment.origin, Bytes::from_static(b"#nose"));
    }

    #[test]
    fn test_from_slice() {
        let fragment = Fragment::from_slice(b"#nose");
        assert_eq!(fragment.origin, Bytes::from_static(b"#nose"));
    }

    #[test]
    fn test_parse() {
        let string = "foo://example.com:8042/over/there?name=ferret#nose";
        let mut cursor = 45;
        let fragment = Fragment::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap();
        assert_eq!(fragment.origin, Bytes::from_static(b"#nose"));
        assert_eq!(cursor, 50);
    }
}
