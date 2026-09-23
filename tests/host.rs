use uniform_resource_identifier::{Host, Uri};

fn ipv4(input: &[u8]) -> bool {
    matches!(Host::from_slice(input).unwrap(), Host::Ipv4(_))
}

fn ipv6(input: &[u8]) -> bool {
    matches!(Host::from_slice(input), Ok(Host::Ipv6(_)))
}

#[test]
fn ipv4_dec_octet() {
    assert!(ipv4(b"0.0.0.0"));
    assert!(ipv4(b"127.0.0.1"));
    assert!(ipv4(b"255.255.255.255"));
    assert!(matches!(
        Host::from_slice(b"01.2.3.4").unwrap(),
        Host::RegName(_)
    ));
    assert!(matches!(
        Host::from_slice(b"256.1.1.1").unwrap(),
        Host::RegName(_)
    ));
    assert!(matches!(
        Host::from_slice(b"1.2.3").unwrap(),
        Host::RegName(_)
    ));
    assert!(matches!(
        Host::from_slice(b"00.0.0.0").unwrap(),
        Host::RegName(_)
    ));
}

#[test]
fn ipv4_in_uri_is_not_reg_name() {
    let uri = Uri::parse_slice(b"http://127.0.0.1/").unwrap();
    assert!(matches!(
        uri.hier_part.authority.unwrap().host,
        Host::Ipv4(_)
    ));
    let reg = Uri::parse_slice(b"http://01.2.3.4/").unwrap();
    assert!(matches!(
        reg.hier_part.authority.unwrap().host,
        Host::RegName(_)
    ));
}

#[test]
fn ipv6_forms() {
    assert!(ipv6(b"::"));
    assert!(ipv6(b"::1"));
    assert!(ipv6(b"1::"));
    assert!(ipv6(b"2001:db8::7"));
    assert!(ipv6(b"2001:DB8::7"));
    assert!(ipv6(b"::ffff:192.0.2.1"));
    assert!(ipv6(b"2001:db8:85a3:0:0:8a2e:370:7334"));
    assert!(ipv6(b"1:2:3:4:5:6:192.0.2.1"));
    assert!(Host::from_slice(b"1:2:3:4:5:6:7").is_err());
    assert!(Host::from_slice(b"1:2:3:4:5:6:7:8:9").is_err());
    assert!(Host::from_slice(b"1::2::3").is_err());
    assert!(Host::from_slice(b":::1").is_err());
    assert!(Host::from_slice(b"1:2:3:4:5:6::192.0.2.1").is_err());
    assert!(Host::from_slice(b"::ffff:192.0.2.01").is_err());
}

#[test]
fn ipv6_literal_in_uri() {
    let uri = Uri::parse_slice(b"http://[2001:db8::7]/c").unwrap();
    assert_eq!(
        uri.hier_part.authority.unwrap().host.bytes().as_ref(),
        b"2001:db8::7"
    );
    let mapped = Uri::parse_slice(b"http://[::ffff:192.0.2.1]").unwrap();
    assert_eq!(
        mapped.hier_part.authority.unwrap().host.bytes().as_ref(),
        b"::ffff:192.0.2.1"
    );
}

#[test]
fn ipv_future_form() {
    assert!(matches!(
        Host::from_slice(b"v1.foo").unwrap(),
        Host::RegName(_)
    ));
    let literal = Uri::parse_slice(b"http://[v1.foo]/a").unwrap();
    assert!(matches!(
        literal.hier_part.authority.unwrap().host,
        Host::IpvFuture(_)
    ));
    assert!(matches!(
        Host::from_slice(b"vF.a:b").unwrap(),
        Host::IpvFuture(_)
    ));
    assert!(Uri::parse_slice(b"http://[v.foo]/").is_err());
    assert!(Uri::parse_slice(b"http://[v1.]/").is_err());
    assert!(Uri::parse_slice(b"http://[V1.foo]/").is_err());
}
