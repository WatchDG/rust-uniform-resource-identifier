use std::error::Error;

pub mod host;
pub mod port;
pub mod userinfo;

pub use host::Host;
pub use port::Port;
pub use userinfo::Userinfo;

#[derive(Debug, Clone)]
pub struct Authority {
    pub userinfo: Option<Userinfo>,
    pub host: Host,
    pub port: Option<Port>,
}

pub fn parse_authority(
    input: &[u8],
    start: &mut usize,
    end: &usize,
) -> Result<Authority, Box<dyn Error>> {
    let mut index = *start;

    let userinfo = userinfo::parse_userinfo(input, &mut index, end)?;

    let host = host::parse_host(input, &mut index, end)?;

    let port = port::parse_port(input, &mut index, end)?;

    *start = index;

    Ok(Authority {
        userinfo,
        host,
        port,
    })
}
