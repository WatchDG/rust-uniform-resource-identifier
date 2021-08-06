use crate::scheme::{get_scheme, parse_scheme};
use crate::Scheme;

use crate::authority::{parse_authority, Authority};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct URI {
    pub scheme: Scheme,
    pub authority: Authority,
}

macro_rules! char_colon {
    () => {
        0x3a
    };
}

macro_rules! char_slash {
    () => {
        0x2f
    };
}

pub fn parse_uri(input: &[u8]) -> Result<URI, Box<dyn Error>> {
    let end = input.len() - 1;
    let mut cursor = 0;

    let scheme = parse_scheme(input, &mut cursor, &end)?;

    // if input[cursor] == '/' and input[cursor+1] == '/'
    cursor += 2;

    let authority = parse_authority(input, &mut cursor, &end)?;

    Ok(URI { scheme, authority })
}
