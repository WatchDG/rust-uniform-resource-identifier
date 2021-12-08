use std::error::Error;

use crate::utils::while_pchar;

#[derive(Debug, Clone, PartialEq)]
pub enum Fragment {
    Fragment(String),
}

pub fn parse_fragment(
    input: &[u8],
    start: &mut usize,
    end: &usize,
) -> Result<Option<Fragment>, Box<dyn Error>> {
    let mut index = *start;

    let fragment = if input[index] == 0x23 {
        index += 1;

        while index <= *end
            && (while_pchar(input, &mut index, end)?
                || input[index] == 0x2f
                || input[index] == 0x3f)
        {
            index += 1;
        }

        let fragment = Fragment::Fragment(String::from_utf8(input[*start + 1..index].to_vec())?);

        *start = index;

        Some(fragment)
    } else {
        None
    };

    Ok(fragment)
}

#[cfg(test)]
mod test_fragment {
    use crate::uri::fragment::{parse_fragment, Fragment};

    #[test]
    fn fragment() {
        {
            let s = b"#test";
            let l = s.len() - 1;
            let f = parse_fragment(s, &mut 0, &l).unwrap();
            assert_eq!(f, Some(Fragment::Fragment(String::from("test"))));
        }
    }
}
