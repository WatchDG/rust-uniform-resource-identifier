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
