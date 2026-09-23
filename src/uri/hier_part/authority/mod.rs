use bytes::{BufMut, Bytes, BytesMut};

use crate::charset::EncodeSet;
use crate::grammar::{
    classify_ip_literal, consume_pct, is_dec_octet, validate_userinfo, IpLiteral,
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

    #[inline]
    pub fn parse(input: &Bytes, start: &mut usize, end: usize) -> Result<Self, UriError> {
        if *start > end || end > input.len() {
            return Err(UriError::InvalidHost);
        }
        let (authority, auth_end) = parse_authority_until(input, *start, end)?;
        *start = auth_end;
        Ok(authority)
    }
}

#[inline]
pub(crate) fn parse_authority(
    input: &Bytes,
    start: usize,
    end: usize,
) -> Result<Authority, UriError> {
    let (authority, consumed) = parse_authority_from(input, start, end, false)?;
    debug_assert_eq!(consumed, end);
    Ok(authority)
}

#[inline]
pub(crate) fn parse_authority_until(
    input: &Bytes,
    start: usize,
    end: usize,
) -> Result<(Authority, usize), UriError> {
    parse_authority_from(input, start, end, true)
}

#[inline]
fn parse_authority_from(
    input: &Bytes,
    start: usize,
    end: usize,
    stop_delims: bool,
) -> Result<(Authority, usize), UriError> {
    if start == end {
        let empty = input.slice(start..start);
        return Ok((
            Authority {
                origin: empty.clone(),
                userinfo: None,
                host: Host::RegName(empty),
                port: None,
            },
            start,
        ));
    }
    let bytes = input.as_ref();
    if bytes[start] == b'[' {
        return parse_leading_bracket(input, start, end, stop_delims);
    }
    parse_reg_authority(input, start, end, stop_delims)
}

fn parse_leading_bracket(
    input: &Bytes,
    start: usize,
    end: usize,
    stop_delims: bool,
) -> Result<(Authority, usize), UriError> {
    let bytes = input.as_ref();
    let mut index = start;
    while index < end {
        let byte = bytes[index];
        if stop_delims && is_hier_end(byte) {
            break;
        }
        if byte == b'%' {
            index = consume_pct(bytes, index, end)?;
        } else if byte == b'@' {
            validate_userinfo(&bytes[start..index])?;
            return parse_after_at(input, start, index, end, stop_delims);
        } else {
            index += 1;
        }
    }
    parse_bracket_host(input, start, None, start, index, false)
}

#[inline]
fn parse_reg_authority(
    input: &Bytes,
    start: usize,
    end: usize,
    stop_delims: bool,
) -> Result<(Authority, usize), UriError> {
    let bytes = input.as_ref();
    let mut index = start;
    let mut colon = None;
    let mut ipv4 = Ipv4Track::new();
    let mut host_bad = false;
    let mut userinfo_bad = false;
    let mut port_bad = false;

    while index < end {
        let byte = bytes[index];
        if stop_delims && is_hier_end(byte) {
            break;
        }
        if byte == b'%' {
            let next = consume_pct(bytes, index, end)?;
            if colon.is_some() {
                port_bad = true;
            } else {
                ipv4.reject();
            }
            index = next;
        } else if byte == b'@' {
            if userinfo_bad {
                return Err(UriError::InvalidUserinfo);
            }
            return parse_after_at(input, start, index, end, stop_delims);
        } else if byte == b':' {
            if colon.is_none() {
                colon = Some(index);
            } else {
                port_bad = true;
            }
            index += 1;
        } else if colon.is_none() {
            if EncodeSet::REG_NAME.contains(byte) {
                ipv4.push(byte);
            } else {
                host_bad = true;
                userinfo_bad = true;
            }
            index += 1;
        } else {
            if !EncodeSet::DIGIT.contains(byte) {
                port_bad = true;
            }
            if !EncodeSet::USERINFO.contains(byte) {
                userinfo_bad = true;
            }
            index += 1;
        }
    }

    if host_bad {
        return Err(UriError::InvalidHost);
    }
    if port_bad {
        return Err(UriError::InvalidPort);
    }
    let host_end = colon.unwrap_or(index);
    let host_bytes = input.slice(start..host_end);
    let host = if ipv4.finish() {
        Host::Ipv4(host_bytes)
    } else {
        Host::RegName(host_bytes)
    };
    let port = colon.map(|colon| Port {
        origin: input.slice(colon + 1..index),
    });
    Ok((
        Authority {
            origin: input.slice(start..index),
            userinfo: None,
            host,
            port,
        },
        index,
    ))
}

fn parse_after_at(
    input: &Bytes,
    origin_start: usize,
    at: usize,
    end: usize,
    stop_delims: bool,
) -> Result<(Authority, usize), UriError> {
    let bytes = input.as_ref();
    let userinfo = Some(Userinfo {
        origin: input.slice(origin_start..at),
    });
    let host_start = at + 1;
    if host_start < end && bytes[host_start] == b'[' {
        return parse_bracket_host(input, origin_start, userinfo, host_start, end, stop_delims);
    }
    let mut index = host_start;
    let mut ipv4 = Ipv4Track::new();
    let mut colon = None;
    while index < end {
        let byte = bytes[index];
        if stop_delims && is_hier_end(byte) {
            break;
        }
        if colon.is_none() {
            if byte == b'%' {
                index = consume_pct(bytes, index, end)?;
                ipv4.reject();
            } else if byte == b':' {
                colon = Some(index);
                index += 1;
            } else if EncodeSet::REG_NAME.contains(byte) {
                ipv4.push(byte);
                index += 1;
            } else {
                return Err(UriError::InvalidHost);
            }
        } else if EncodeSet::DIGIT.contains(byte) {
            index += 1;
        } else {
            return Err(UriError::InvalidPort);
        }
    }
    let host_end = colon.unwrap_or(index);
    let host_bytes = input.slice(host_start..host_end);
    let host = if ipv4.finish() {
        Host::Ipv4(host_bytes)
    } else {
        Host::RegName(host_bytes)
    };
    let port = colon.map(|colon| Port {
        origin: input.slice(colon + 1..index),
    });
    Ok((
        Authority {
            origin: input.slice(origin_start..index),
            userinfo,
            host,
            port,
        },
        index,
    ))
}

fn parse_bracket_host(
    input: &Bytes,
    origin_start: usize,
    userinfo: Option<Userinfo>,
    bracket: usize,
    end: usize,
    stop_delims: bool,
) -> Result<(Authority, usize), UriError> {
    let bytes = input.as_ref();
    let mut close = bracket + 1;
    while close < end {
        let byte = bytes[close];
        if stop_delims && is_hier_end(byte) {
            return Err(UriError::InvalidHost);
        }
        if byte == b']' {
            break;
        }
        close += 1;
    }
    if close >= end || bytes[close] != b']' {
        return Err(UriError::InvalidHost);
    }
    let inner = input.slice(bracket + 1..close);
    let host = match classify_ip_literal(inner.as_ref())? {
        IpLiteral::Ipv6 => Host::Ipv6(inner),
        IpLiteral::IpvFuture => Host::IpvFuture(inner),
    };
    let after = close + 1;
    if after >= end || (stop_delims && is_hier_end(bytes[after])) {
        let consumed = if after < end { after } else { end };
        return Ok((
            Authority {
                origin: input.slice(origin_start..consumed),
                userinfo,
                host,
                port: None,
            },
            consumed,
        ));
    }
    if bytes[after] != b':' {
        return Err(UriError::InvalidHost);
    }
    let mut port_end = after + 1;
    while port_end < end {
        let byte = bytes[port_end];
        if stop_delims && is_hier_end(byte) {
            break;
        }
        if !EncodeSet::DIGIT.contains(byte) {
            return Err(UriError::InvalidPort);
        }
        port_end += 1;
    }
    Ok((
        Authority {
            origin: input.slice(origin_start..port_end),
            userinfo,
            host,
            port: Some(Port {
                origin: input.slice(after + 1..port_end),
            }),
        },
        port_end,
    ))
}

#[inline]
fn is_hier_end(byte: u8) -> bool {
    byte == b'/' || byte == b'?' || byte == b'#'
}

struct Ipv4Track {
    possible: bool,
    dots: u8,
    octet: [u8; 3],
    len: u8,
}

impl Ipv4Track {
    #[inline]
    fn new() -> Self {
        Self {
            possible: true,
            dots: 0,
            octet: [0; 3],
            len: 0,
        }
    }

    #[inline]
    fn reject(&mut self) {
        self.possible = false;
    }

    #[inline]
    fn push(&mut self, byte: u8) {
        if !self.possible {
            return;
        }
        if byte == b'.' {
            if self.dots >= 3 || !is_dec_octet(&self.octet[..self.len as usize]) {
                self.possible = false;
                return;
            }
            self.dots += 1;
            self.len = 0;
            return;
        }
        if !EncodeSet::DIGIT.contains(byte) || self.len >= 3 {
            self.possible = false;
            return;
        }
        self.octet[self.len as usize] = byte;
        self.len += 1;
    }

    #[inline]
    fn finish(&self) -> bool {
        self.possible && self.dots == 3 && is_dec_octet(&self.octet[..self.len as usize])
    }
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
