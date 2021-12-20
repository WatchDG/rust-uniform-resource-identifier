use std::error::Error;

use crate::utils::{while_ip_v4_address, while_reg_name};

#[derive(Debug, Clone, PartialEq)]
pub enum Host {
    Host(String),
    Ipv4Addr(String),
}

pub fn parse_host(input: &[u8], start: &mut usize, end: &usize) -> Result<Host, Box<dyn Error>> {
    let mut index = *start;

    if while_ip_v4_address(input, &mut index, end)? {
        let string = String::from_utf8(input[*start..index - 1].to_vec())?;
        *start = index;
        return Ok(Host::Ipv4Addr(string));
    }

    if while_reg_name(input, &mut index, end)? {
        let string = String::from_utf8(input[*start..index].to_vec())?;
        *start = index;
        return Ok(Host::Host(string));
    }

    Ok(Host::Host("".into()))
}

#[cfg(test)]
mod test {
    use crate::uri::authority::host::{parse_host, Host};

    #[test]
    fn valid_ip_v4_address() {
        let string = b"127.0.0.1:80";
        let mut cursor = 0;
        let end = string.len() - 1;

        let host = parse_host(string, &mut cursor, &end).unwrap();

        assert_eq!(host, Host::Ipv4Addr("127.0.0.1".into()));
    }
}
