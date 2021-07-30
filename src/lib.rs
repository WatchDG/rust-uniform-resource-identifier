pub enum Scheme {
    Http,
    Https,
}

pub struct Port(usize);

impl Port {
    pub fn new(port: usize) -> Self {
        Port(port)
    }
}

#[cfg(test)]
mod test_port {
    use crate::Port;

    #[test]
    fn new_port() {
        let port = Port::new(80);
        assert_eq!(port.0, 80);
    }
}
