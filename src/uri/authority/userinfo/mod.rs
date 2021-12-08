use std::error::Error;

use crate::utils::while_pct_encoded;
use crate::{is_sub_delims, is_unreserved};

#[derive(Debug, Clone, PartialEq)]
pub enum Userinfo {
    Userinfo(String),
}

pub fn parse_userinfo(
    input: &[u8],
    start: &mut usize,
    end: &usize,
) -> Result<Option<Userinfo>, Box<dyn Error>> {
    let mut index = *start;

    while index < *end
        && (is_unreserved!(input[index])
            || while_pct_encoded(input, &mut index, end)?
            || is_sub_delims!(input[index])
            || input[index] == 0x3a)
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

#[cfg(test)]
mod test {
    use crate::uri::authority::userinfo::{parse_userinfo, Userinfo};

    #[test]
    fn parse_userinfo_1() {
        let string = b"user:password@";
        let end = string.len() - 1;
        let mut cursor = 0;

        let userinfo = parse_userinfo(string, &mut cursor, &end).unwrap();

        assert_eq!(userinfo, Some(Userinfo::Userinfo("user:password".into())));
        assert_eq!(cursor, 14);
    }
}
