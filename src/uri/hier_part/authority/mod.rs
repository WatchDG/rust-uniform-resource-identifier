use bytes::Bytes;

use crate::UriError;

pub struct Authority {
    origin: Bytes,
}

impl Authority {
    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.origin.clone()
    }

    #[inline]
    pub fn from_bytes(input: Bytes) -> Self {
        Self { origin: input }
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Self {
        let bytes = Bytes::copy_from_slice(input);
        Self { origin: bytes }
    }

    pub fn parse(input: &[u8], start: &mut usize, end: &usize) -> Result<Self, UriError> {
        let mut index = *start;
        while index < *end && input[index] != 0x2f {
            index += 1;
        }
        let value = Self::from_slice(&input[*start..index]);
        *start = index;
        Ok(value)
    }
}

// use std::error::Error;

// pub mod host;
// pub mod port;
// pub mod userinfo;

// pub use host::Host;
// pub use port::Port;
// pub use userinfo::Userinfo;

// #[derive(Debug, Clone, PartialEq)]
// pub struct Authority {
// pub userinfo: Option<Userinfo>,
// pub host: Host,
// pub port: Option<Port>,
// }

// pub fn parse_authority(
//     input: &[u8],
//     start: &mut usize,
//     end: &usize,
// ) -> Result<Authority, Box<dyn Error>> {
//     let mut index = *start;

//     let userinfo = userinfo::parse_userinfo(input, &mut index, end)?;

//     let host = host::parse_host(input, &mut index, end)?;

//     let port = port::parse_port(input, &mut index, end)?;

//     *start = index;

//     Ok(Authority {
//         userinfo,
//         host,
//         port,
//     })
// }
