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
    pub authority: Option<Authority>,
    pub path: Path,
    pub query: Option<Query>,
    pub fragment: Option<Fragment>,
}

pub fn parse_uri(input: &[u8]) -> Result<URI, Box<dyn Error>> {
    let end = input.len() - 1;
    let mut cursor = 0;

    let scheme = parse_scheme(input, &mut cursor, &end)?;

    let (authority, path, query, fragment) =
        if cursor + 1 <= end && input[cursor] == 0x2f && input[cursor + 1] == 0x2f {
            cursor += 2;
            let authority = parse_authority(input, &mut cursor, &end)?;
            let path = parse_path(input, &mut cursor, &end)?;
            let query = parse_query(input, &mut cursor, &end)?;
            let fragment = parse_fragment(input, &mut &mut cursor, &end)?;
            (Some(authority), path, query, fragment)
        } else {
            let query = parse_query(input, &mut cursor, &end)?;
            let fragment = parse_fragment(input, &mut &mut cursor, &end)?;
            (None, Path::Path(String::from("")), query, fragment)
        };

    Ok(URI {
        scheme,
        authority,
        path,
        query,
        fragment,
    })
}
