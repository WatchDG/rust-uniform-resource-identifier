use bytes::{BufMut, Bytes, BytesMut};

use crate::grammar::{classify_unbracketed, is_ipv4, is_ipv6, is_ipv_future, UnbracketedHost};
use crate::UriError;

#[derive(Debug, Clone, PartialEq)]
pub enum Host {
    Ipv4(Bytes),
    Ipv6(Bytes),
    IpvFuture(Bytes),
    RegName(Bytes),
}

impl Host {
    #[inline]
    pub fn bytes(&self) -> Bytes {
        match self {
            Host::Ipv4(bytes)
            | Host::Ipv6(bytes)
            | Host::IpvFuture(bytes)
            | Host::RegName(bytes) => bytes.clone(),
        }
    }

    #[inline]
    pub fn from_bytes(input: Bytes) -> Result<Self, UriError> {
        Ok(match classify_unbracketed(&input)? {
            UnbracketedHost::Ipv4 => Host::Ipv4(input),
            UnbracketedHost::Ipv6 => Host::Ipv6(input),
            UnbracketedHost::IpvFuture => Host::IpvFuture(input),
            UnbracketedHost::RegName => Host::RegName(input),
        })
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Result<Self, UriError> {
        Self::from_bytes(Bytes::copy_from_slice(input))
    }
}

pub(crate) fn validate_host(host: &Host) -> Result<(), UriError> {
    match host {
        Host::Ipv4(bytes) => {
            if is_ipv4(bytes) {
                Ok(())
            } else {
                Err(UriError::InvalidHost)
            }
        }
        Host::Ipv6(bytes) => {
            if is_ipv6(bytes) {
                Ok(())
            } else {
                Err(UriError::InvalidIpv6)
            }
        }
        Host::IpvFuture(bytes) => {
            if is_ipv_future(bytes) {
                Ok(())
            } else {
                Err(UriError::InvalidHost)
            }
        }
        Host::RegName(bytes) => {
            use crate::charset::EncodeSet;
            use crate::grammar::scan_allowed;
            match scan_allowed(bytes, 0, bytes.len(), EncodeSet::REG_NAME) {
                Ok(end) if end == bytes.len() => Ok(()),
                Err(error) => Err(error),
                Ok(_) => Err(UriError::InvalidHost),
            }
        }
    }
}

pub(crate) fn host_wire_len(host: &Host) -> usize {
    match host {
        Host::Ipv4(bytes) | Host::RegName(bytes) => bytes.len(),
        Host::Ipv6(bytes) | Host::IpvFuture(bytes) => bytes.len() + 2,
    }
}

pub(crate) fn write_host(host: &Host, out: &mut BytesMut) {
    match host {
        Host::Ipv4(bytes) | Host::RegName(bytes) => out.put_slice(bytes),
        Host::Ipv6(bytes) | Host::IpvFuture(bytes) => {
            out.put_u8(b'[');
            out.put_slice(bytes);
            out.put_u8(b']');
        }
    }
}
