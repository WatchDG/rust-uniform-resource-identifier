use lazy_static::lazy_static;
use std::error::Error;
use std::fmt;

use crate::Port;

#[derive(Debug, Clone)]
pub enum SchemeError {
    InvalidScheme,
    UnknownScheme,
}

impl fmt::Display for SchemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemeError::InvalidScheme => write!(f, "Invalid scheme."),
            SchemeError::UnknownScheme => write!(f, "Unknown scheme."),
        }
    }
}

impl Error for SchemeError {}

#[derive(Debug, Clone, PartialEq)]
pub enum Scheme {
    Http,
    Https,
    Ws,
    Wss,
    File,
    Mailto,
}

impl From<Scheme> for Port {
    fn from(scheme: Scheme) -> Port {
        match scheme {
            Scheme::Http => Port::Http,
            Scheme::Https => Port::Https,
            _ => panic!(""),
        }
    }
}

impl ToString for Scheme {
    fn to_string(&self) -> String {
        match self {
            Scheme::Http => String::from("http"),
            Scheme::Https => String::from("https"),
            Scheme::Ws => String::from("ws"),
            Scheme::Wss => String::from("wss"),
            Scheme::File => String::from("file"),
            Scheme::Mailto => String::from("mailto"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SchemeChar {
    code: u8,
    next_char: Vec<SchemeChar>,
    scheme: Option<Scheme>,
}

fn add_scheme(mut chars: &mut Vec<SchemeChar>, value: &[u8], enum_value: Scheme) {
    let end_idx = value.len() - 1;

    for (idx, code) in value.to_ascii_lowercase().iter().enumerate() {
        let mut search_result = chars.binary_search_by(|x| (x.code).cmp(code));
        if search_result.is_err() {
            let scheme = if idx == end_idx {
                Some(enum_value.clone())
            } else {
                None
            };
            chars.push(SchemeChar {
                code: *code,
                next_char: Vec::new(),
                scheme,
            });
            chars.sort_unstable_by(|a, b| a.code.cmp(&b.code));
            search_result = chars.binary_search_by(|x| (x.code).cmp(code));
        }
        let index = search_result.unwrap();
        chars = &mut chars[index].next_char;
    }
}

lazy_static! {
    pub static ref SCHEME_CHARS: Vec<SchemeChar> = {
        let mut chars = Vec::<SchemeChar>::new();
        add_scheme(&mut chars, b"http", Scheme::Http);
        add_scheme(&mut chars, b"https", Scheme::Https);
        add_scheme(&mut chars, b"ws", Scheme::Ws);
        add_scheme(&mut chars, b"wss", Scheme::Wss);
        add_scheme(&mut chars, b"file", Scheme::File);
        add_scheme(&mut chars, b"mailto", Scheme::Mailto);
        chars
    };
}

pub fn get_scheme(value: &[u8]) -> Result<Scheme, SchemeError> {
    let mut chars = &*SCHEME_CHARS;
    let end_idx = value.len() - 1;
    for (idx, code) in value.to_ascii_lowercase().iter().enumerate() {
        let search_result = chars.binary_search_by(|header_char| (header_char.code).cmp(code));
        let index = search_result.map_err(|_| SchemeError::UnknownScheme)?;
        if idx == end_idx {
            return chars[index]
                .scheme
                .clone()
                .ok_or(SchemeError::UnknownScheme);
        }
        chars = &chars[index].next_char;
    }
    Err(SchemeError::UnknownScheme)
}

pub fn parse_scheme(
    input: &[u8],
    start: &mut usize,
    end: &usize,
) -> Result<Scheme, Box<dyn Error>> {
    let mut index = *start;

    while index < *end && input[index] != 0x3a {
        index += 1;
    }

    if input[index] != 0x3a {
        return Err(SchemeError::InvalidScheme.into());
    }

    let scheme = get_scheme(&input[*start..index])?;

    *start = index + 1;

    Ok(scheme)
}

#[cfg(test)]
mod test_scheme {
    use crate::uri::scheme::parse_scheme;
    use crate::{Port, Scheme};

    #[test]
    fn scheme_to_port() {
        let scheme = Scheme::Http;
        let port: Port = scheme.into();
        assert_eq!(port, Port::Http);
    }

    #[test]
    fn scheme_to_string() {
        let scheme = Scheme::Http;
        assert_eq!(scheme.to_string(), "http");
    }

    #[test]
    #[should_panic]
    fn panic_parse_scheme_1() {
        let string = b"http";
        let end = string.len() - 1;
        let mut cursor = 0;
        parse_scheme(string, &mut cursor, &end).unwrap();
    }

    #[test]
    #[should_panic(expected = "UnknownScheme")]
    fn panic_parse_scheme_2() {
        let string = b"abc:";
        let end = string.len() - 1;
        let mut cursor = 0;
        parse_scheme(string, &mut cursor, &end).unwrap();
    }

    #[test]
    fn parse_scheme_1() {
        let string = b"https:";
        let mut cursor = 0;
        let end = string.len() - 1;

        let scheme = parse_scheme(string, &mut cursor, &end).unwrap();

        assert_eq!(scheme, Scheme::Https);
        assert_eq!(cursor, 6);
    }

    #[test]
    fn parse_scheme_2() {
        let string = b"https://example.com";
        let mut cursor = 0;
        let end = string.len() - 1;

        let scheme = parse_scheme(string, &mut cursor, &end).unwrap();

        assert_eq!(scheme, Scheme::Https);
        assert_eq!(cursor, 6);
    }

    #[test]
    fn parse_scheme_3() {
        let string = b"mailto:someone@example.com";
        let mut cursor = 0;
        let end = string.len() - 1;

        let scheme = parse_scheme(string, &mut cursor, &end).unwrap();

        assert_eq!(scheme, Scheme::Mailto);
        assert_eq!(cursor, 7);
    }
}
