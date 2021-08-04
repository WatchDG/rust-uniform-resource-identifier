use crate::scheme::get_scheme;
use crate::Scheme;

use std::error::Error;

#[derive(Debug, Clone)]
pub struct URI {
    pub scheme: Scheme,
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
    let end_idx = input.len() - 1;

    let mut s_idx = 0;
    let mut e_idx = 0;
    while e_idx < end_idx && input[e_idx] != char_colon!() {
        e_idx += 1;
    }

    let scheme = get_scheme(&input[s_idx..e_idx])?;
    s_idx = end_idx;

    Ok(URI { scheme })
}
