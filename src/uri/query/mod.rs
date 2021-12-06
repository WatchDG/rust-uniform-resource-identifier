use std::error::Error;

use crate::utils::while_pchar;

#[derive(Debug, Clone, PartialEq)]
pub enum Query {
    Query(String),
}

pub fn parse_query(
    input: &[u8],
    start: &mut usize,
    end: &usize,
) -> Result<Option<Query>, Box<dyn Error>> {
    let mut index = *start;

    let query = if input[index] == 0x3f {
        index += 1;
        while index <= *end
            && (while_pchar(input, &mut index, end)?
                || input[index] == 0x2f
                || input[index] == 0x3f)
        {
            index += 1;
        }
        let q = Query::Query(String::from_utf8(input[*start + 1..index].to_vec())?);
        *start = index;
        Some(q)
    } else {
        None
    };

    Ok(query)
}
