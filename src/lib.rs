mod macros;
mod uri;
mod utils;

pub use uri::{Fragment, HierPart, Query, Scheme, Uri, UriBuilder};

#[derive(Debug)]
pub enum UriError {
    InvalidScheme,
    UnknownScheme,
    InvalidQuery,
    InvalidUri,
}

impl std::fmt::Display for UriError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UriError::InvalidScheme => {
                write!(f, "Invalid scheme.")
            }
            UriError::UnknownScheme => {
                write!(f, "Unknown scheme.")
            }
            UriError::InvalidQuery => {
                write!(f, "Invalid query.")
            }
            UriError::InvalidUri => {
                write!(f, "Invalid URI.")
            }
        }
    }
}

impl std::error::Error for UriError {}
