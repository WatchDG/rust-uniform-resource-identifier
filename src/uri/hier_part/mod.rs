use bytes::Bytes;

use crate::UriError;

mod authority;

use authority::Authority;

#[derive(Debug, Clone, PartialEq)]
pub struct HierPart(Bytes);

impl HierPart {
    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.0.clone()
    }

    #[inline]
    pub fn from_bytes(input: Bytes) -> Self {
        Self(input)
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Self {
        let bytes = Bytes::copy_from_slice(input);
        Self(bytes)
    }

    pub fn parse(input: &[u8], start: &mut usize, end: &usize) -> Result<Self, UriError> {
        let mut index = *start;
        while index < *end && input[index] != 0x3f && input[index] != 0x23 {
            index += 1;
        }
        let value = Self::from_slice(&input[*start..index]);
        *start = index;
        Ok(value)
    }
}

#[cfg(test)]
mod tests_hier_part {
    use crate::HierPart;
    use bytes::Bytes;

    #[test]
    fn test_parse() {
        let string = "foo://example.com:8042/over/there?name=ferret#nose";
        let mut cursor = 4;
        let hier_part = HierPart::parse(string.as_bytes(), &mut cursor, &string.len()).unwrap();
        assert_eq!(
            hier_part.bytes(),
            Bytes::from_static(b"//example.com:8042/over/there")
        );
        assert_eq!(cursor, 33);
    }
}
