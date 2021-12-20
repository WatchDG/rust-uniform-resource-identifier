use std::error::Error;

use crate::{is_hexdig, is_sub_delims, is_unreserved};

#[derive(Debug, Clone)]
pub enum PctEncodedError {
    InvalidValue,
}

impl std::fmt::Display for PctEncodedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid value")
    }
}

impl Error for PctEncodedError {}

pub fn while_pct_encoded(
    input: &[u8],
    start: &mut usize,
    end: &usize,
) -> Result<bool, Box<dyn Error>> {
    let mut index = *start;
    let flag = input[index] == 0x25;

    while index <= *end && input[index] == 0x25 {
        if index + 2 > *end || !is_hexdig!(input[index + 1]) || !is_hexdig!(input[index + 2]) {
            return Err(PctEncodedError::InvalidValue.into());
        }
        index += 3;
    }

    if flag {
        *start = index - 1;
    }

    Ok(flag)
}

pub fn while_pchar(input: &[u8], start: &mut usize, end: &usize) -> Result<bool, Box<dyn Error>> {
    let mut index = *start;
    let flag = is_unreserved!(input[index])
        || while_pct_encoded(input, &mut index, end)?
        || is_sub_delims!(input[index])
        || input[index] == 0x3a
        || input[index] == 0x40;

    while index < *end
        && (is_unreserved!(input[index])
            || while_pct_encoded(input, &mut index, end)?
            || is_sub_delims!(input[index])
            || input[index] == 0x3a
            || input[index] == 0x40)
    {
        index += 1;
    }

    Ok(flag)
}

pub fn while_ip_v4_address(
    input: &[u8],
    start: &mut usize,
    end: &usize,
) -> Result<bool, Box<dyn Error>> {
    if *end - *start < 6 || *end - *start > 14 {
        return Ok(false);
    }

    let mut index = *start;
    let mut dec_octet_count = 0;
    let mut dec_octet_index = index;

    while dec_octet_count < 4 {
        while dec_octet_index <= *end
            && dec_octet_index - index < 4
            && input[dec_octet_index] != 0x2e
            && input[dec_octet_index] >= 0x30
            && input[dec_octet_index] <= 0x39
        {
            dec_octet_index += 1;
        }

        match dec_octet_index - index {
            3 => {
                if !((input[index] == 0x31
                    && input[index + 1] >= 0x30
                    && input[index + 1] <= 0x39
                    && input[index + 2] >= 0x30
                    && input[index + 2] <= 0x39)
                    || (input[index] == 0x32
                        && input[index + 1] >= 0x30
                        && input[index + 1] <= 0x35
                        && input[index + 2] >= 0x30
                        && input[index + 2] <= 0x35))
                {
                    return Ok(false);
                }
            }
            2 => {
                if input[index] < 0x31
                    || input[index] > 0x39
                    || input[index + 1] < 0x30
                    || input[index + 1] > 0x39
                {
                    return Ok(false);
                }
            }
            1 => {
                if input[index] < 0x30 || input[index] > 0x39 {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        }

        dec_octet_index += 1;
        index = dec_octet_index;
        dec_octet_count += 1;
    }

    *start = dec_octet_index;

    Ok(true)
}

#[cfg(test)]
mod test_while_ip_v4_address {
    use crate::utils::while_ip_v4_address;

    #[test]
    fn invalid_length_short() {
        let string = b"0.0.0.";
        let mut cursor = 0;
        let end = string.len() - 1;
        assert_eq!(
            while_ip_v4_address(string, &mut cursor, &end).unwrap(),
            false
        );
        assert_eq!(cursor, 0);
    }

    #[test]
    fn invalid_length_long() {
        let string = b"255.255.255.2550";
        let mut cursor = 0;
        let end = string.len() - 1;
        assert_eq!(
            while_ip_v4_address(string, &mut cursor, &end).unwrap(),
            false
        );
        assert_eq!(cursor, 0);
    }

    #[test]
    fn invalid_dec_octet_length_short() {
        let string = b"255.255.255.";
        let mut cursor = 0;
        let end = string.len() - 1;
        assert_eq!(
            while_ip_v4_address(string, &mut cursor, &end).unwrap(),
            false
        );
        assert_eq!(cursor, 0);
    }

    #[test]
    fn invalid_dec_octet_length_long() {
        let string = b"255.255.0.2555";
        let mut cursor = 0;
        let end = string.len() - 1;
        assert_eq!(
            while_ip_v4_address(string, &mut cursor, &end).unwrap(),
            false
        );
        assert_eq!(cursor, 0);
    }

    #[test]
    fn invalid_dec_octet_value() {
        let string = b"255.255.255.256";
        let mut cursor = 0;
        let end = string.len() - 1;
        assert_eq!(
            while_ip_v4_address(string, &mut cursor, &end).unwrap(),
            false
        );
        assert_eq!(cursor, 0);
    }

    #[test]
    fn valid() {
        let string = b"0.10.200.200:80";
        let mut cursor = 0;
        let end = string.len() - 1;
        assert_eq!(
            while_ip_v4_address(string, &mut cursor, &end).unwrap(),
            true
        );
        // assert_eq!(cursor, 12);
    }
}

pub fn while_number(input: &[u8], start: &mut usize, end: &usize) -> Result<bool, Box<dyn Error>> {
    let mut index = *start;

    if index > *end || input[index] < 0x31 || input[index] > 0x39 {
        return Ok(false);
    }

    index += 1;

    while index <= *end && input[index] >= 0x30 && input[index] <= 0x39 {
        index += 1;
    }

    *start = index;

    return Ok(true);
}

#[cfg(test)]
mod test_while_number {
    use crate::utils::while_number;

    #[test]
    fn valid() {
        let string = b"80/";
        let mut cursor = 0;
        let end = string.len() - 1;

        assert_eq!(while_number(string, &mut cursor, &end).unwrap(), true);
        assert_eq!(cursor, 2);
    }
}

pub fn while_reg_name(
    input: &[u8],
    start: &mut usize,
    end: &usize,
) -> Result<bool, Box<dyn Error>> {
    let mut index = *start;

    while index <= *end
        && (is_unreserved!(input[index])
            || while_pct_encoded(input, &mut index, end)?
            || is_sub_delims!(input[index]))
    {
        index += 1;
    }

    Ok(if index != *start {
        *start = index;
        true
    } else {
        false
    })
}
