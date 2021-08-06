use std::error::Error;

pub mod host;
pub mod port;

pub use host::Host;
pub use port::Port;

#[derive(Debug, Clone)]
pub struct Authority {
    pub host: Host,
    pub port: Option<Port>,
}

macro_rules! char_colon {
    () => {
        0x3a
    };
}
macro_rules! char_slash {
    () => {
        0x2f
    };
}

pub fn parse_authority(
    input: &[u8],
    start: &mut usize,
    end: &usize,
) -> Result<Authority, Box<dyn Error>> {
    let mut e_idx = *start;
    while e_idx < *end && input[e_idx] != char_slash!() {
        e_idx += 1;
    }

    if input[e_idx] == char_slash!() {
        e_idx -= 1;
    }

    let mut m_idx = *start;

    while m_idx < e_idx && input[m_idx] != char_colon!() {
        m_idx += 1;
    }

    let (host, port) = if m_idx != e_idx {
        let h = Host::Host(String::from_utf8(input[*start..m_idx].to_vec())?);
        let p: usize = String::from_utf8(input[(m_idx + 1)..=e_idx].to_vec())?.parse()?;
        (h, Some(Port::Port(p)))
    } else {
        let h = Host::Host(String::from_utf8(input[*start..=m_idx].to_vec())?);
        (h, None)
    };

    *start = e_idx;

    Ok(Authority { host, port })
}
