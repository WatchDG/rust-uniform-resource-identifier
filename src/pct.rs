use bytes::{BufMut, Bytes, BytesMut};

use crate::charset::EncodeSet;
use crate::UriError;

const HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";

#[inline]
pub fn encoded_len(input: &[u8], set: EncodeSet) -> usize {
    let mut len = 0;
    for &byte in input {
        if set.contains(byte) {
            len += 1;
        } else {
            len += 3;
        }
    }
    len
}

#[inline]
fn needs_encode(input: &[u8], set: EncodeSet) -> bool {
    input.iter().any(|&byte| !set.contains(byte))
}

pub fn encode_into(input: &[u8], set: EncodeSet, out: &mut BytesMut) {
    out.reserve(encoded_len(input, set));
    for &byte in input {
        if set.contains(byte) {
            out.put_u8(byte);
        } else {
            out.put_u8(b'%');
            out.put_u8(HEX_UPPER[(byte >> 4) as usize]);
            out.put_u8(HEX_UPPER[(byte & 0x0f) as usize]);
        }
    }
}

/// Percent-encode `input`. Returns the same buffer when every byte is allowed.
pub fn percent_encode(input: &Bytes, set: EncodeSet) -> Bytes {
    if !needs_encode(input, set) {
        return input.clone();
    }
    let mut out = BytesMut::with_capacity(encoded_len(input, set));
    encode_into(input, set, &mut out);
    out.freeze()
}

#[inline]
fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

pub(crate) fn decoded_len(input: &[u8]) -> Result<usize, UriError> {
    let mut len = 0;
    let mut index = 0;
    while index < input.len() {
        if input[index] == b'%' {
            if index + 2 >= input.len()
                || hex_value(input[index + 1]).is_none()
                || hex_value(input[index + 2]).is_none()
            {
                return Err(UriError::InvalidPctEncoded);
            }
            index += 3;
        } else {
            index += 1;
        }
        len += 1;
    }
    Ok(len)
}

pub fn decode_into(input: &[u8], out: &mut BytesMut) -> Result<(), UriError> {
    let len = decoded_len(input)?;
    out.reserve(len);
    let mut index = 0;
    while index < input.len() {
        if input[index] == b'%' {
            let high = hex_value(input[index + 1]).ok_or(UriError::InvalidPctEncoded)?;
            let low = hex_value(input[index + 2]).ok_or(UriError::InvalidPctEncoded)?;
            out.put_u8((high << 4) | low);
            index += 3;
        } else {
            out.put_u8(input[index]);
            index += 1;
        }
    }
    Ok(())
}

/// Percent-decode `input`. Returns the same buffer when it contains no `%`.
pub fn percent_decode(input: &Bytes) -> Result<Bytes, UriError> {
    if !input.contains(&b'%') {
        return Ok(input.clone());
    }
    let mut out = BytesMut::new();
    decode_into(input, &mut out)?;
    Ok(out.freeze())
}
