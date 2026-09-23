use bytes::Bytes;
use uniform_resource_identifier::{
    Authority, Fragment, HierPart, Host, Path, Port, Query, Scheme, Userinfo,
};

#[test]
fn scheme_bytes_and_parse() {
    let scheme = Scheme::from_bytes(Bytes::from_static(b"foo")).unwrap();
    assert_eq!(scheme.bytes(), Bytes::from_static(b"foo"));
    assert_eq!(scheme.origin, Bytes::from_static(b"foo"));
    assert_eq!(
        Scheme::from_slice(b"foo").unwrap().origin,
        Bytes::from_static(b"foo")
    );

    let bytes = Bytes::from_static(b"foo://example.com:8042/over/there?name=ferret#nose");
    let mut cursor = 0;
    let parsed = Scheme::parse(&bytes, &mut cursor, bytes.len()).unwrap();
    assert_eq!(parsed.origin, Bytes::from_static(b"foo"));
    assert_eq!(cursor, 4);

    assert!(Scheme::from_slice(b"").is_err());
    assert!(Scheme::from_slice(b"1http").is_err());
    assert!(Scheme::from_slice(b"http:").is_err());
}

#[test]
fn path_bytes_and_validation() {
    let path = Path::from_bytes(Bytes::from_static(b"/over/there")).unwrap();
    assert_eq!(path.bytes(), Bytes::from_static(b"/over/there"));
    assert_eq!(path.origin, Bytes::from_static(b"/over/there"));
    assert_eq!(
        Path::from_slice(b"/over/there").unwrap().origin,
        Bytes::from_static(b"/over/there")
    );
    assert!(Path::from_slice(b"/a b").is_err());
    assert!(Path::from_slice(b"/%ZZ").is_err());
    assert!(Path::from_slice(b"/a%20b").is_ok());
}

#[test]
fn query_bytes_and_parse() {
    let query = Query::from_bytes(Bytes::from_static(b"name=ferret")).unwrap();
    assert_eq!(query.bytes(), Bytes::from_static(b"name=ferret"));
    assert_eq!(query.origin, Bytes::from_static(b"name=ferret"));
    assert_eq!(
        Query::from_slice(b"name=ferret").unwrap().origin,
        Bytes::from_static(b"name=ferret")
    );

    let bytes = Bytes::from_static(b"foo://example.com:8042/over/there?name=ferret#nose");
    let mut cursor = 33;
    let parsed = Query::parse(&bytes, &mut cursor, bytes.len()).unwrap();
    assert_eq!(parsed.bytes(), Bytes::from_static(b"name=ferret"));
    assert_eq!(cursor, 45);
}

#[test]
fn fragment_bytes_and_parse() {
    let fragment = Fragment::from_bytes(Bytes::from_static(b"nose")).unwrap();
    assert_eq!(fragment.bytes(), Bytes::from_static(b"nose"));
    assert_eq!(fragment.origin, Bytes::from_static(b"nose"));
    assert_eq!(
        Fragment::from_slice(b"nose").unwrap().origin,
        Bytes::from_static(b"nose")
    );

    let bytes = Bytes::from_static(b"foo://example.com:8042/over/there?name=ferret#nose");
    let mut cursor = 45;
    let parsed = Fragment::parse(&bytes, &mut cursor, bytes.len()).unwrap();
    assert_eq!(parsed.origin, Bytes::from_static(b"nose"));
    assert_eq!(cursor, 50);
}

#[test]
fn authority_bytes_and_parse() {
    let authority = Authority::from_bytes(Bytes::from_static(b"example.com")).unwrap();
    assert_eq!(authority.bytes(), Bytes::from_static(b"example.com"));
    assert_eq!(authority.origin, Bytes::from_static(b"example.com"));
    assert!(matches!(authority.host, Host::RegName(_)));
    assert_eq!(
        Authority::from_slice(b"example.com").unwrap().origin,
        Bytes::from_static(b"example.com")
    );

    let bytes = Bytes::from_static(b"foo://example.com:8042/over/there?name=ferret#nose");
    let mut cursor = 6;
    let parsed = Authority::parse(&bytes, &mut cursor, bytes.len()).unwrap();
    assert_eq!(parsed.bytes(), Bytes::from_static(b"example.com:8042"));
    assert_eq!(parsed.host.bytes().as_ref(), b"example.com");
    assert_eq!(parsed.port.unwrap().origin.as_ref(), b"8042");
    assert_eq!(cursor, 22);
}

#[test]
fn host_stored_form() {
    assert!(matches!(
        Host::from_slice(b"example.com").unwrap(),
        Host::RegName(_)
    ));
    assert!(matches!(
        Host::from_slice(b"127.0.0.1").unwrap(),
        Host::Ipv4(_)
    ));
    assert!(matches!(
        Host::from_slice(b"2001:db8::7").unwrap(),
        Host::Ipv6(_)
    ));
    assert!(Host::from_slice(b"exa mple").is_err());
}

#[test]
fn port_digits_and_empty() {
    assert_eq!(Port::from_slice(b"8042").unwrap().origin.as_ref(), b"8042");
    assert!(Port::from_slice(b"").is_ok());
    assert!(Port::from_slice(b"80a").is_err());
}

#[test]
fn userinfo_allows_colon_and_pct() {
    assert_eq!(
        Userinfo::from_slice(b"user:pass").unwrap().origin.as_ref(),
        b"user:pass"
    );
    assert!(Userinfo::from_slice(b"a b").is_err());
    assert!(Userinfo::from_slice(b"a%20b").is_ok());
}

#[test]
fn hier_part_parse() {
    let hier_part =
        HierPart::from_bytes(Bytes::from_static(b"//example.com:8042/over/there")).unwrap();
    assert_eq!(
        hier_part.bytes(),
        Bytes::from_static(b"//example.com:8042/over/there")
    );
    assert_eq!(
        hier_part.origin,
        Bytes::from_static(b"//example.com:8042/over/there")
    );
    assert!(matches!(
        hier_part.authority.unwrap().host,
        Host::RegName(_)
    ));
    assert_eq!(
        HierPart::from_slice(b"//example.com:8042/over/there")
            .unwrap()
            .path
            .origin
            .as_ref(),
        b"/over/there"
    );

    let bytes = Bytes::from_static(b"foo://example.com:8042/over/there?name=ferret#nose");
    let mut cursor = 4;
    let parsed = HierPart::parse(&bytes, &mut cursor, bytes.len()).unwrap();
    assert_eq!(
        parsed.bytes(),
        Bytes::from_static(b"//example.com:8042/over/there")
    );
    assert_eq!(parsed.path, Path::from_slice(b"/over/there").unwrap());
    assert_eq!(cursor, 33);
}
