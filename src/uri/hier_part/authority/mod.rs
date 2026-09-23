use bytes::{BufMut, Bytes, BytesMut};

use crate::grammar::{
    classify_ip_literal, classify_reg_host, find_at, validate_port, validate_userinfo, IpLiteral,
    UnbracketedHost,
};
use crate::UriError;

use self::host::{host_wire_len, write_host};

mod host;
mod port;
mod userinfo;

pub(crate) use host::validate_host;
pub use host::Host;
pub use port::Port;
pub use userinfo::Userinfo;

#[derive(Debug, Clone, PartialEq)]
pub struct Authority {
    pub origin: Bytes,
    pub userinfo: Option<Userinfo>,
    pub host: Host,
    pub port: Option<Port>,
}

impl Authority {
    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.origin.clone()
    }

    #[inline]
    pub fn from_bytes(input: Bytes) -> Result<Self, UriError> {
        parse_authority(&input, 0, input.len())
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Result<Self, UriError> {
        Self::from_bytes(Bytes::copy_from_slice(input))
    }

    pub fn parse(input: &Bytes, start: &mut usize, end: usize) -> Result<Self, UriError> {
        if *start > end || end > input.len() {
            return Err(UriError::InvalidHost);
        }
        let bytes = input.as_ref();
        let mut auth_end = *start;
        while auth_end < end
            && bytes[auth_end] != b'/'
            && bytes[auth_end] != b'?'
            && bytes[auth_end] != b'#'
        {
            auth_end += 1;
        }
        let authority = parse_authority(input, *start, auth_end)?;
        *start = auth_end;
        Ok(authority)
    }
}

pub(crate) fn parse_authority(
    input: &Bytes,
    start: usize,
    end: usize,
) -> Result<Authority, UriError> {
    let bytes = input.as_ref();
    let at = find_at(bytes, start, end)?;
    let (userinfo, host_start) = if let Some(at) = at {
        validate_userinfo(&bytes[start..at])?;
        (
            Some(Userinfo {
                origin: input.slice(start..at),
            }),
            at + 1,
        )
    } else {
        (None, start)
    };

    if host_start < end && bytes[host_start] == b'[' {
        let mut close = host_start + 1;
        while close < end && bytes[close] != b']' {
            close += 1;
        }
        if close >= end {
            return Err(UriError::InvalidHost);
        }
        let inner = input.slice(host_start + 1..close);
        let host = match classify_ip_literal(inner.as_ref())? {
            IpLiteral::Ipv6 => Host::Ipv6(inner),
            IpLiteral::IpvFuture => Host::IpvFuture(inner),
        };
        let port = port_after_bracket(input, close + 1, end)?;
        return Ok(Authority {
            origin: input.slice(start..end),
            userinfo,
            host,
            port,
        });
    }

    let mut host_end = host_start;
    while host_end < end && bytes[host_end] != b':' {
        host_end += 1;
    }
    let host_bytes = input.slice(host_start..host_end);
    let host = match classify_reg_host(host_bytes.as_ref())? {
        UnbracketedHost::Ipv4 => Host::Ipv4(host_bytes),
        UnbracketedHost::RegName => Host::RegName(host_bytes),
        UnbracketedHost::Ipv6 | UnbracketedHost::IpvFuture => return Err(UriError::InvalidHost),
    };
    let port = if host_end < end {
        let digits = input.slice(host_end + 1..end);
        validate_port(digits.as_ref())?;
        Some(Port { origin: digits })
    } else {
        None
    };
    Ok(Authority {
        origin: input.slice(start..end),
        userinfo,
        host,
        port,
    })
}

fn port_after_bracket(input: &Bytes, start: usize, end: usize) -> Result<Option<Port>, UriError> {
    if start == end {
        return Ok(None);
    }
    if input[start] != b':' {
        return Err(UriError::InvalidHost);
    }
    let digits = input.slice(start + 1..end);
    validate_port(digits.as_ref())?;
    Ok(Some(Port { origin: digits }))
}

pub(crate) fn authority_wire_len(authority: &Authority) -> usize {
    let mut len = host_wire_len(&authority.host);
    if let Some(userinfo) = &authority.userinfo {
        len += userinfo.origin.len() + 1;
    }
    if let Some(port) = &authority.port {
        len += 1 + port.origin.len();
    }
    len
}

pub(crate) fn write_authority(authority: &Authority, out: &mut BytesMut) {
    if let Some(userinfo) = &authority.userinfo {
        out.put_slice(&userinfo.origin);
        out.put_u8(b'@');
    }
    write_host(&authority.host, out);
    if let Some(port) = &authority.port {
        out.put_u8(b':');
        out.put_slice(&port.origin);
    }
}
