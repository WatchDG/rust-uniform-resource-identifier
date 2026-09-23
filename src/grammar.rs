use crate::charset::EncodeSet;
use crate::UriError;

#[inline]
pub(crate) fn consume_pct(input: &[u8], index: usize, end: usize) -> Result<usize, UriError> {
    if index + 2 >= end
        || !EncodeSet::HEXDIG.contains(input[index + 1])
        || !EncodeSet::HEXDIG.contains(input[index + 2])
    {
        return Err(UriError::InvalidPctEncoded);
    }
    Ok(index + 3)
}

/// Consume `pct-encoded` or bytes in `set`. Stops before the first other byte.
pub(crate) fn scan_allowed(
    input: &[u8],
    mut index: usize,
    end: usize,
    set: EncodeSet,
) -> Result<usize, UriError> {
    while index < end {
        if input[index] == b'%' {
            index = consume_pct(input, index, end)?;
        } else if set.contains(input[index]) {
            index += 1;
        } else {
            break;
        }
    }
    Ok(index)
}

fn expect_allowed(input: &[u8], set: EncodeSet, err: UriError) -> Result<(), UriError> {
    match scan_allowed(input, 0, input.len(), set) {
        Ok(end) if end == input.len() => Ok(()),
        Ok(_) => Err(err),
        Err(error) => Err(error),
    }
}

pub(crate) fn validate_scheme(input: &[u8]) -> Result<(), UriError> {
    if input.is_empty() || !EncodeSet::ALPHA.contains(input[0]) {
        return Err(UriError::InvalidScheme);
    }
    if input[1..]
        .iter()
        .all(|&byte| EncodeSet::SCHEME.contains(byte))
    {
        Ok(())
    } else {
        Err(UriError::InvalidScheme)
    }
}

pub(crate) fn validate_userinfo(input: &[u8]) -> Result<(), UriError> {
    expect_allowed(input, EncodeSet::USERINFO, UriError::InvalidUserinfo)
}

pub(crate) fn validate_port(input: &[u8]) -> Result<(), UriError> {
    if input.iter().all(|&byte| EncodeSet::DIGIT.contains(byte)) {
        Ok(())
    } else {
        Err(UriError::InvalidPort)
    }
}

pub(crate) fn validate_query(input: &[u8]) -> Result<(), UriError> {
    expect_allowed(input, EncodeSet::QUERY, UriError::InvalidQuery)
}

pub(crate) fn validate_fragment(input: &[u8]) -> Result<(), UriError> {
    expect_allowed(input, EncodeSet::FRAGMENT, UriError::InvalidFragment)
}

pub(crate) fn validate_path_value(input: &[u8]) -> Result<(), UriError> {
    if input.is_empty() {
        return Ok(());
    }
    let end = if input[0] == b'/' {
        parse_path_abempty(input, 0, input.len())?
    } else {
        parse_path_rootless(input, 0, input.len())?
    };
    if end == input.len() {
        Ok(())
    } else {
        Err(UriError::InvalidPath)
    }
}

/// Index of `:` when `input` starts with a scheme. `None` when it is a relative reference.
pub(crate) fn scheme_colon(input: &[u8]) -> Option<usize> {
    if input.is_empty() || !EncodeSet::ALPHA.contains(input[0]) {
        return None;
    }
    let mut index = 1;
    while index < input.len() && EncodeSet::SCHEME.contains(input[index]) {
        index += 1;
    }
    if index < input.len() && input[index] == b':' {
        Some(index)
    } else {
        None
    }
}

pub(crate) fn find_query_or_fragment(input: &[u8], start: usize, end: usize) -> usize {
    let mut index = start;
    while index < end && input[index] != b'?' && input[index] != b'#' {
        index += 1;
    }
    index
}

pub(crate) fn find_at(input: &[u8], start: usize, end: usize) -> Result<Option<usize>, UriError> {
    let mut index = start;
    while index < end {
        if input[index] == b'%' {
            index = consume_pct(input, index, end)?;
        } else if input[index] == b'@' {
            return Ok(Some(index));
        } else {
            index += 1;
        }
    }
    Ok(None)
}

pub(crate) fn is_ipv4(input: &[u8]) -> bool {
    let mut parts = 0u8;
    let mut start = 0usize;
    for (index, &byte) in input.iter().enumerate() {
        if byte == b'.' {
            if !is_dec_octet(&input[start..index]) {
                return false;
            }
            parts += 1;
            start = index + 1;
            if parts > 3 {
                return false;
            }
        } else if !byte.is_ascii_digit() {
            return false;
        }
    }
    parts == 3 && is_dec_octet(&input[start..])
}

fn is_dec_octet(input: &[u8]) -> bool {
    match input {
        [digit] if digit.is_ascii_digit() => true,
        [tens, ones] if (b'1'..=b'9').contains(tens) && ones.is_ascii_digit() => true,
        [b'1', tens, ones] if tens.is_ascii_digit() && ones.is_ascii_digit() => true,
        [b'2', tens, ones] if (b'0'..=b'4').contains(tens) && ones.is_ascii_digit() => true,
        [b'2', b'5', ones] if (b'0'..=b'5').contains(ones) => true,
        _ => false,
    }
}

pub(crate) fn is_ipv6(input: &[u8]) -> bool {
    if input.is_empty() {
        return false;
    }
    let mut index = 0;
    let mut groups: u8 = 0;
    let mut compressed = false;

    if input.starts_with(b"::") {
        compressed = true;
        index = 2;
        if index == input.len() {
            return true;
        }
    }

    loop {
        if index >= input.len() {
            break;
        }
        let start = index;
        let mut hex_len = 0u8;
        while index < input.len() && hex_len < 4 && EncodeSet::HEXDIG.contains(input[index]) {
            index += 1;
            hex_len += 1;
        }
        if hex_len == 0 {
            return false;
        }
        if index < input.len() && input[index] == b'.' {
            if !is_ipv4(&input[start..]) {
                return false;
            }
            groups = groups.saturating_add(2);
            break;
        }
        groups = groups.saturating_add(1);
        if groups > 8 {
            return false;
        }
        if index == input.len() {
            break;
        }
        if input[index] != b':' {
            return false;
        }
        if index + 1 < input.len() && input[index + 1] == b':' {
            if compressed {
                return false;
            }
            compressed = true;
            index += 2;
            if index == input.len() {
                break;
            }
        } else {
            index += 1;
            if index == input.len() {
                return false;
            }
        }
    }

    if compressed {
        groups < 8
    } else {
        groups == 8
    }
}

pub(crate) fn is_ipv_future(input: &[u8]) -> bool {
    if input.len() < 4 || input[0] != b'v' {
        return false;
    }
    let mut index = 1;
    let hex_start = index;
    while index < input.len() && EncodeSet::HEXDIG.contains(input[index]) {
        index += 1;
    }
    if index == hex_start || index >= input.len() || input[index] != b'.' {
        return false;
    }
    index += 1;
    if index >= input.len() {
        return false;
    }
    while index < input.len() {
        if !EncodeSet::IPV_FUTURE_TAIL.contains(input[index]) {
            return false;
        }
        index += 1;
    }
    true
}

pub(crate) enum UnbracketedHost {
    Ipv4,
    Ipv6,
    IpvFuture,
    RegName,
}

pub(crate) fn classify_unbracketed(input: &[u8]) -> Result<UnbracketedHost, UriError> {
    if is_ipv4(input) {
        return Ok(UnbracketedHost::Ipv4);
    }
    match scan_allowed(input, 0, input.len(), EncodeSet::REG_NAME) {
        Ok(end) if end == input.len() => return Ok(UnbracketedHost::RegName),
        Err(error) => return Err(error),
        Ok(_) => {}
    }
    if is_ipv6(input) {
        return Ok(UnbracketedHost::Ipv6);
    }
    if is_ipv_future(input) {
        return Ok(UnbracketedHost::IpvFuture);
    }
    if input.contains(&b':') {
        Err(UriError::InvalidIpv6)
    } else {
        Err(UriError::InvalidHost)
    }
}

/// Host text that sits between `[` and `]` in an authority.
pub(crate) fn classify_reg_host(input: &[u8]) -> Result<UnbracketedHost, UriError> {
    if is_ipv4(input) {
        return Ok(UnbracketedHost::Ipv4);
    }
    match scan_allowed(input, 0, input.len(), EncodeSet::REG_NAME) {
        Ok(end) if end == input.len() => Ok(UnbracketedHost::RegName),
        Err(error) => Err(error),
        Ok(_) => Err(UriError::InvalidHost),
    }
}

pub(crate) enum IpLiteral {
    Ipv6,
    IpvFuture,
}

pub(crate) fn classify_ip_literal(input: &[u8]) -> Result<IpLiteral, UriError> {
    if is_ipv6(input) {
        return Ok(IpLiteral::Ipv6);
    }
    if is_ipv_future(input) {
        return Ok(IpLiteral::IpvFuture);
    }
    if input.contains(&b':') {
        Err(UriError::InvalidIpv6)
    } else {
        Err(UriError::InvalidHost)
    }
}

pub(crate) fn parse_path_abempty(
    input: &[u8],
    mut index: usize,
    end: usize,
) -> Result<usize, UriError> {
    while index < end {
        if input[index] != b'/' {
            return Err(UriError::InvalidPath);
        }
        index += 1;
        index = scan_allowed(input, index, end, EncodeSet::SEGMENT)?;
        if index < end && input[index] != b'/' {
            return Err(UriError::InvalidPath);
        }
    }
    Ok(index)
}

pub(crate) fn parse_path_absolute(
    input: &[u8],
    mut index: usize,
    end: usize,
) -> Result<usize, UriError> {
    if index >= end || input[index] != b'/' {
        return Err(UriError::InvalidPath);
    }
    index += 1;
    if index == end {
        return Ok(index);
    }
    let segment_start = index;
    index = scan_allowed(input, index, end, EncodeSet::SEGMENT)?;
    if index == segment_start {
        return Err(UriError::InvalidPath);
    }
    if index < end && input[index] != b'/' {
        return Err(UriError::InvalidPath);
    }
    while index < end {
        if input[index] != b'/' {
            return Err(UriError::InvalidPath);
        }
        index += 1;
        index = scan_allowed(input, index, end, EncodeSet::SEGMENT)?;
        if index < end && input[index] != b'/' {
            return Err(UriError::InvalidPath);
        }
    }
    Ok(index)
}

pub(crate) fn parse_path_rootless(
    input: &[u8],
    mut index: usize,
    end: usize,
) -> Result<usize, UriError> {
    let start = index;
    index = scan_allowed(input, index, end, EncodeSet::SEGMENT)?;
    if index == start {
        return Err(UriError::InvalidPath);
    }
    finish_path_segments(input, index, end)
}

pub(crate) fn parse_path_noscheme(
    input: &[u8],
    mut index: usize,
    end: usize,
) -> Result<usize, UriError> {
    let start = index;
    index = scan_allowed(input, index, end, EncodeSet::SEGMENT_NC)?;
    if index == start || (index < end && input[index] != b'/') {
        return Err(UriError::InvalidPath);
    }
    finish_path_segments(input, index, end)
}

fn finish_path_segments(input: &[u8], mut index: usize, end: usize) -> Result<usize, UriError> {
    if index < end && input[index] != b'/' {
        return Err(UriError::InvalidPath);
    }
    while index < end {
        if input[index] != b'/' {
            return Err(UriError::InvalidPath);
        }
        index += 1;
        index = scan_allowed(input, index, end, EncodeSet::SEGMENT)?;
        if index < end && input[index] != b'/' {
            return Err(UriError::InvalidPath);
        }
    }
    Ok(index)
}
