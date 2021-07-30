#[derive(Debug, PartialEq)]
pub enum Scheme {
    Http,
    Https,
    Scheme(String),
}

impl From<Scheme> for Port {
    fn from(scheme: Scheme) -> Port {
        match scheme {
            Scheme::Http => Port::Http,
            Scheme::Https => Port::Https,
            _ => panic!(""),
        }
    }
}

impl ToString for Scheme {
    fn to_string(&self) -> String {
        match self {
            Scheme::Http => String::from("http"),
            Scheme::Https => String::from("https"),
            Scheme::Scheme(scheme) => scheme.clone(),
        }
    }
}

#[cfg(test)]
mod test_scheme {
    use crate::{Port, Scheme};

    #[test]
    fn scheme_to_port() {
        let scheme = Scheme::Http;
        let port: Port = scheme.into();
        assert_eq!(port, Port::Http);
    }

    #[test]
    fn scheme_to_string() {
        let scheme = Scheme::Http;
        assert_eq!(scheme.to_string(), "http");
    }
}

#[derive(Debug, PartialEq)]
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
