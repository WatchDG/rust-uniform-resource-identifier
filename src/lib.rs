mod charset;
mod grammar;
mod pct;
mod uri;

pub use charset::EncodeSet;
pub use pct::{decode_into, encode_into, encoded_len, percent_decode, percent_encode};
pub use uri::{
    encode_pairs, Authority, Fragment, HierPart, HierPartBuilder, Host, Path, Port, Query,
    QueryPairs, Scheme, Uri, UriBuilder, Userinfo,
};

#[derive(Debug)]
pub enum UriError {
    InvalidScheme,
    InvalidUserinfo,
    InvalidHost,
    InvalidIpv6,
    InvalidPort,
    InvalidPath,
    InvalidQuery,
    InvalidFragment,
    InvalidPctEncoded,
    InvalidUri,
}

impl std::fmt::Display for UriError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UriError::InvalidScheme => write!(f, "Invalid scheme."),
            UriError::InvalidUserinfo => write!(f, "Invalid userinfo."),
            UriError::InvalidHost => write!(f, "Invalid host."),
            UriError::InvalidIpv6 => write!(f, "Invalid IPv6 address."),
            UriError::InvalidPort => write!(f, "Invalid port."),
            UriError::InvalidPath => write!(f, "Invalid path."),
            UriError::InvalidQuery => write!(f, "Invalid query."),
            UriError::InvalidFragment => write!(f, "Invalid fragment."),
            UriError::InvalidPctEncoded => write!(f, "Invalid percent-encoding."),
            UriError::InvalidUri => write!(f, "Invalid URI."),
        }
    }
}

impl std::error::Error for UriError {}
