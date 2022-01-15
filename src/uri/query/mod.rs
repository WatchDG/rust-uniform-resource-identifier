use bytes::Bytes;

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
        while index < *end && input[index] != 0x23 {
            index += 1;
        }
        let value = Self::from_slice(&input[*start..index]);
        *start = index;
        Ok(value)
    }
}

#[cfg(test)]
mod tests_query {
    use crate::Query;
    use bytes::Bytes;

    #[test]
    fn parse() {
        let string = "foo://example.com:8042/over/there?name=ferret#nose";
        let mut cursor = 33;
        let query = Query::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap();
        assert_eq!(query.bytes(), Bytes::from_static(b"?name=ferret"));
        assert_eq!(cursor, 45);
    }
}
