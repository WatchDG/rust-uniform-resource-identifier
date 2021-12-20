use std::error::Error;
use std::fmt;

use crate::utils::while_number;
use crate::Scheme;

#[derive(Debug, Clone)]
pub enum PortError {
    PortNotExists,
}

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortError::PortNotExists => write!(f, "Port not exists."),
        }
    }
}

impl Error for PortError {}

#[derive(Debug, Clone, PartialEq)]
pub enum Port {
    Http,
    Https,
    Port(String),
}

impl From<Port> for Scheme {
    fn from(port: Port) -> Scheme {
        match port {
            Port::Http => Scheme::Http,
            Port::Https => Scheme::Https,
            _ => panic!("cast port {:?} to scheme", port),
        }
    }
}

impl ToString for Port {
    fn to_string(&self) -> String {
        match self {
            Port::Http => String::from("80"),
            Port::Https => String::from("443"),
            Port::Port(port) => port.clone().to_string(),
        }
    }
}

pub fn parse_port(
    input: &[u8],
    start: &mut usize,
    end: &usize,
) -> Result<Option<Port>, Box<dyn Error>> {
    let mut index = *start;

    if index > *end || input[index] != 0x3a {
        return Ok(None);
    }

    index += 1;

    if !while_number(input, &mut index, end)? {
        return Err(PortError::PortNotExists.into());
    };

    let string = String::from_utf8(input[*start + 1..index].to_vec())?;
    let port = Port::Port(string);

    *start = index;

    return Ok(Some(port));
}

#[cfg(test)]
mod test_port {
    use crate::{Port, Scheme};

    #[test]
    fn port_to_scheme() {
        let port = Port::Http;
        let scheme: Scheme = port.into();
        assert_eq!(scheme, Scheme::Http);
    }

    #[test]
    fn port_to_string() {
        let port = Port::Http;
        assert_eq!(port.to_string(), "80");
    }
}

#[cfg(test)]
mod test_parse_port {
    use crate::{uri::authority::port::parse_port, Port};

    #[test]
    fn valid() {
        let string = b":80/path";
        let mut cursor = 0;
        let end = string.len() - 1;

        let port = parse_port(string, &mut cursor, &end).unwrap();

        assert_eq!(port, Some(Port::Port("80".into())));
        assert_eq!(cursor, 3);
    }
}
