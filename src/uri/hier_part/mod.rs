use bytes::Bytes;

use crate::UriError;

mod authority;
mod path;

pub use authority::Authority;
pub use authority::Host;
pub use path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct HierPart {
    pub authority: Option<Authority>,
    pub path: Option<Path>,
    pub origin: Bytes,
}

impl HierPart {
    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.origin.clone()
    }

    #[inline]
    pub fn from_bytes(input: Bytes) -> Self {
        Self {
            authority: None,
            path: None,
            origin: input,
        }
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Self {
        let bytes = Bytes::copy_from_slice(input);
        Self {
            authority: None,
            path: None,
            origin: bytes,
        }
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
    fn test_bytes() {
        let hier_part = HierPart::from_bytes(Bytes::from_static(b"//example.com:8042/over/there"));
        assert_eq!(
            hier_part.bytes(),
            Bytes::from_static(b"//example.com:8042/over/there")
        );
    }

    #[test]
    fn test_from_bytes() {
        let hier_part = HierPart::from_bytes(Bytes::from_static(b"//example.com:8042/over/there"));
        assert_eq!(
            hier_part.origin,
            Bytes::from_static(b"//example.com:8042/over/there")
        );
    }

    #[test]
    fn test_from_slice() {
        let hier_part = HierPart::from_slice(b"//example.com:8042/over/there");
        assert_eq!(
            hier_part.bytes(),
            Bytes::from_static(b"//example.com:8042/over/there")
        );
    }

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

#[derive(Debug, Clone, PartialEq)]
pub struct HierPartBuilder {
    pub authority: Option<Authority>,
    pub path: Option<Path>,
}

impl HierPartBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            authority: None,
            path: None,
        }
    }

    pub fn authority(&mut self, authority: Authority) -> &Self {
        self.authority = Some(authority);
        self
    }

    pub fn path(&mut self, path: Path) -> &Self {
        self.path = Some(path);
        self
    }

    pub fn build(&self) -> HierPart {
        match (&self.authority, &self.path) {
            (Some(authority), Some(path)) => {
                let origin = Bytes::new();
                HierPart {
                    authority: self.authority.clone(),
                    path: self.path.clone(),
                    origin,
                }
            }
            (None, Some(path)) => {
                let origin = path.origin.clone();
                HierPart {
                    authority: None,
                    path: self.path.clone(),
                    origin,
                }
            }
            _ => {
                panic!("");
            }
        }
    }
}
