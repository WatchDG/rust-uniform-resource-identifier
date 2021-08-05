use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Path {
    Path(String),
}

macro_rules! char_question {
    () => {
        0x3f
    };
}

pub fn parse_path(input: &[u8], start: &mut usize, end: &usize) -> Result<Path, Box<dyn Error>> {
    let mut index = *start;

    while index < *end && input[index] != char_question!() {
        index += 1;
    }

    if input[index] == char_question!() {
        index -= 1;
    }

    let s = String::from_utf8(input[*start..=index].to_vec())?;
    let p = Path::Path(s);

    *start = index + 1;

    Ok(p)
}

#[cfg(test)]
mod test_path {

    use crate::path::{parse_path, Path};

    #[test]
    fn test_parse_path_1() {
        use parse_path;
        use Path;

        let s = b"/path/to/file";
        let l = s.len() - 1;
        let mut c = 0;

        let p = parse_path(s, &mut c, &l).unwrap();

        assert_eq!(p, Path::Path(String::from("/path/to/file")));
        assert_eq!(c, 13);
    }

    #[test]
    fn test_parse_path_2() {
        use parse_path;
        use Path;

        let s = b"/path/to/file?v=1";
        let l = s.len() - 1;
        let mut c = 0;

        let p = parse_path(s, &mut c, &l).unwrap();

        assert_eq!(p, Path::Path(String::from("/path/to/file")));
        assert_eq!(c, 13);
    }
}
