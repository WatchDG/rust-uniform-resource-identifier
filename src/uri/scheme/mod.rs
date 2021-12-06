use lazy_static::lazy_static;
use std::error::Error;
use std::fmt;

use crate::Port;

#[derive(Debug, Clone)]
pub enum SchemeError {
    InvalidScheme,
}

impl fmt::Display for SchemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemeError::InvalidScheme => write!(f, "Invalid scheme."),
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
    Scheme(String),
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
            Scheme::Scheme(scheme) => scheme.clone(),
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
        chars
    };
}

pub fn get_scheme(value: &[u8]) -> Result<Scheme, SchemeError> {
    let mut chars = &*SCHEME_CHARS;
    let end_idx = value.len() - 1;
    for (idx, code) in value.to_ascii_lowercase().iter().enumerate() {
        let search_result = chars.binary_search_by(|header_char| (header_char.code).cmp(code));
        let index = search_result.map_err(|_| SchemeError::InvalidScheme)?;
        if idx == end_idx {
            return chars[index]
                .scheme
                .clone()
                .ok_or(SchemeError::InvalidScheme);
        }
        chars = &chars[index].next_char;
    }
    Err(SchemeError::InvalidScheme)
}

macro_rules! char_colon {
    () => {
        0x3a
    };
}

pub fn parse_scheme(
    input: &[u8],
    start: &mut usize,
    end: &usize,
) -> Result<Scheme, Box<dyn Error>> {
    let mut index = *start;

    while index < *end
        && ((input[index] >= 0x41 && input[index] <= 0x5a)
            || (input[index] >= 0x61 && input[index] <= 0x7a))
    {
        index += 1
    }

    if input[index] == char_colon!() {
        index -= 1;
    }

    let s = get_scheme(&input[*start..=index])?;

    *start = index + 2;

    Ok(s)
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
    fn test_parse_scheme_1() {
        use parse_scheme;

        let s = b"https";
        let l = s.len() - 1;
        let mut c = 0;
        let scheme = parse_scheme(s, &mut c, &l).unwrap();

        assert_eq!(scheme, Scheme::Https);
        assert_eq!(c, 6);
    }

    #[test]
    fn test_parse_scheme_2() {
        use parse_scheme;

        let s = b"https://";
        let l = s.len() - 1;
        let mut c = 0;
        let scheme = parse_scheme(s, &mut c, &l).unwrap();

        assert_eq!(scheme, Scheme::Https);
        assert_eq!(c, 6);
    }
}
