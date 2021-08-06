use crate::Scheme;

#[derive(Debug, Clone, PartialEq)]
pub enum Port {
    Http,
    Https,
    Port(usize),
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
