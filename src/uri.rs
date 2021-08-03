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

pub fn parse_uri(input: &[u8]) -> Result<URI, Box<dyn Error>> {
    let end_idx = input.len() - 1;

    let mut idx = 0;
    while idx < end_idx && input[idx] != char_colon!() {
        idx += 1;
    }

    let scheme = get_scheme(&input[0..idx])?;
    Ok(URI { scheme })
}
