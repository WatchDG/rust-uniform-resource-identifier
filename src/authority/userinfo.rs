use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Userinfo {
    Userinfo(String),
}

macro_rules! is_alpha {
    ($char: expr) => {
        (($char >= 0x41 && $char <= 0x5a) || ($char >= 0x61 && $char <= 0x7a))
    };
}

macro_rules! is_sub_delims {
    ($char: expr) => {
        ($char == 0x21
            || $char == 0x24
            || ($char >= 0x26 && $char <= 0x2c)
            || $char == 0x3b
            || $char == 0x3d)
    };
}

pub fn parse_userinfo(
    input: &[u8],
    start: &mut usize,
    end: &usize,
) -> Result<Option<Userinfo>, Box<dyn Error>> {
    let mut index = *start;

    while index < *end
        && (is_alpha!(input[index]) || is_sub_delims!(input[index]) || input[index] == 0x3a)
    {
        index += 1;
    }

    Ok(if input[index] == 0x40 {
        let userinfo = Userinfo::Userinfo(String::from_utf8(input[*start..index].to_vec())?);
        *start = index + 1;
        Some(userinfo)
    } else {
        None
    })
}
