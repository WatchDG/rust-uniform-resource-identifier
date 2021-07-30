#[derive(Debug, PartialEq)]
pub enum Scheme {
    Http,
    Https,
}

impl From<Scheme> for Port {
    fn from(scheme: Scheme) -> Port {
        match scheme {
            Scheme::Http => Port(80),
            Scheme::Https => Port(443),
        }
    }
}

impl ToString for Scheme {
    fn to_string(&self) -> String {
        match self {
            Scheme::Http => String::from("http"),
            Scheme::Https => String::from("https"),
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
        assert_eq!(port.0, 80);
    }

    #[test]
    fn scheme_to_string() {
        let scheme = Scheme::Http;
        assert_eq!(scheme.to_string(), "http");
    }
}

pub struct Port(usize);

impl Port {
    pub fn new(port: usize) -> Self {
        Port(port)
    }
}

impl From<Port> for Scheme {
    fn from(port: Port) -> Scheme {
        match port.0 {
            80 => Scheme::Http,
            443 => Scheme::Https,
            _ => panic!("cast port {} to scheme", port.0),
        }
    }
}

impl ToString for Port {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[cfg(test)]
mod test_port {
    use crate::{Port, Scheme};

    #[test]
    fn new_port() {
        let port = Port::new(80);
        assert_eq!(port.0, 80);
    }

    #[test]
    fn port_to_scheme() {
        let port = Port::new(80);
        let scheme: Scheme = port.into();
        assert_eq!(scheme, Scheme::Http);
    }

    #[test]
    fn port_to_string() {
        let port = Port::new(80);
        assert_eq!(port.to_string(), "80");
    }
}
