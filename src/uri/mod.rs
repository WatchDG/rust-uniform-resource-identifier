use std::error::Error;

pub mod authority;
pub mod fragment;
pub mod path;
pub mod query;
pub mod scheme;

use authority::{parse_authority, Authority};
use fragment::{parse_fragment, Fragment};
use path::{parse_path, Path};
use query::{parse_query, Query};
use scheme::{parse_scheme, Scheme};

#[derive(Debug, Clone)]
pub struct URI {
    pub scheme: Scheme,
    pub authority: Authority,
    pub path: Path,
    pub query: Option<Query>,
    pub fragment: Option<Fragment>,
}

pub fn parse_uri(input: &[u8]) -> Result<URI, Box<dyn Error>> {
    let end = input.len() - 1;
    let mut cursor = 0;

    let scheme = parse_scheme(input, &mut cursor, &end)?;

    // if input[cursor] == '/' and input[cursor+1] == '/'
    cursor += 2;

    let authority = parse_authority(input, &mut cursor, &end)?;

    let path = parse_path(input, &mut cursor, &end)?;

    let query = parse_query(input, &mut cursor, &end)?;

    let fragment = parse_fragment(input, &mut &mut cursor, &end)?;

    Ok(URI {
        scheme,
        authority,
        path,
        query,
        fragment,
    })
}
