use bytes::Bytes;

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub origin: Bytes,
}

impl Path {
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
mod tests_path {
    use crate::Path;
    use bytes::Bytes;

    #[test]
    fn test_bytes() {
        let path = Path::from_bytes(Bytes::from_static(b"/over/there"));
        assert_eq!(path.bytes(), Bytes::from_static(b"/over/there"));
    }

    #[test]
    fn test_from_bytes() {
        let path = Path::from_bytes(Bytes::from_static(b"/over/there"));
        assert_eq!(path.origin, Bytes::from_static(b"/over/there"));
    }

    #[test]
    fn test_from_slice() {
        let path = Path::from_slice(b"/over/there");
        assert_eq!(path.origin, Bytes::from_static(b"/over/there"));
    }
}

// use std::error::Error;
//
// use crate::is_unreserved;
//
// #[derive(Debug, Clone, PartialEq)]
// pub enum Path {
//     Path(String),
//     PathEmpty,
// }
//
// pub fn parse_path(input: &[u8], start: &mut usize, end: &usize) -> Result<Path, Box<dyn Error>> {
//     let mut index = *start;
//
//     while index <= *end && input[index] == 0x2f {
//         index += 1;
//         while index <= *end && is_unreserved!(input[index]) {
//             index += 1;
//         }
//     }
//
//     let string = String::from_utf8(input[*start..index].to_vec())?;
//     let path = Path::Path(string);
//
//     *start = index;
//
//     Ok(path)
// }
//
// #[cfg(test)]
// mod test_path {
//
//     use crate::uri::path::{parse_path, Path};
//
//     #[test]
//     fn test_parse_path_1() {
//         use parse_path;
//         use Path;
//
//         let s = b"/path/to/file";
//         let l = s.len() - 1;
//         let mut c = 0;
//
//         let p = parse_path(s, &mut c, &l).unwrap();
//
//         assert_eq!(p, Path::Path(String::from("/path/to/file")));
//         assert_eq!(c, 13);
//     }
//
//     #[test]
//     fn test_parse_path_2() {
//         use parse_path;
//         use Path;
//
//         let s = b"/path/to/file?v=1";
//         let l = s.len() - 1;
//         let mut c = 0;
//
//         let p = parse_path(s, &mut c, &l).unwrap();
//
//         assert_eq!(p, Path::Path(String::from("/path/to/file")));
//         assert_eq!(c, 13);
//     }
// }
