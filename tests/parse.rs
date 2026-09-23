use bytes::Bytes;
use uniform_resource_identifier::{
    Authority, Fragment, HierPart, HierPartBuilder, Host, Path, Query, Scheme, Uri, UriBuilder,
    Userinfo,
};

const EXAMPLE: &[u8] = b"foo://example.com:8042/over/there?name=ferret#nose";

#[test]
fn parse_slice_keeps_origin() {
    let uri = Uri::parse_slice(EXAMPLE).unwrap();
    assert_eq!(uri.bytes(), Bytes::from_static(EXAMPLE));
}

#[test]
fn parse_shares_component_bytes() {
    let origin = Bytes::copy_from_slice(EXAMPLE);
    let base = origin.as_ptr();
    let uri = Uri::parse(origin).unwrap();
    assert_eq!(uri.scheme.as_ref().unwrap().origin.as_ref(), b"foo");
    assert_eq!(uri.scheme.as_ref().unwrap().origin.as_ptr(), base);
    assert_eq!(uri.query.as_ref().unwrap().origin.as_ref(), b"name=ferret");
    assert_eq!(uri.query.as_ref().unwrap().origin.as_ptr(), unsafe {
        base.add(34)
    });
    assert_eq!(uri.fragment.as_ref().unwrap().origin.as_ref(), b"nose");
    assert_eq!(uri.hier_part.path.origin.as_ref(), b"/over/there");
    let authority = uri.hier_part.authority.unwrap();
    assert!(authority.userinfo.is_none());
    assert_eq!(authority.host.bytes().as_ref(), b"example.com");
    assert!(matches!(authority.host, Host::RegName(_)));
    assert_eq!(authority.port.unwrap().origin.as_ref(), b"8042");
}

#[test]
fn builder_parse_and_build() {
    let bytes = Bytes::from_static(EXAMPLE);
    let mut cursor = 0;
    let parsed = UriBuilder::parse(&bytes, &mut cursor, bytes.len()).unwrap();
    assert_eq!(cursor, 50);

    let mut reference = UriBuilder::new();
    reference.scheme(Scheme::from_slice(b"foo").unwrap());
    reference.hier_part(HierPart::from_slice(b"//example.com:8042/over/there").unwrap());
    reference.query(Query::from_slice(b"name=ferret").unwrap());
    reference.fragment(Fragment::from_slice(b"nose").unwrap());
    assert_eq!(parsed, reference);

    let built = parsed.build().unwrap();
    assert_eq!(built.bytes().as_ref(), EXAMPLE);
}

#[test]
fn build_scheme_only() {
    let uri = UriBuilder::new()
        .scheme(Scheme::from_slice(b"http").unwrap())
        .build()
        .unwrap();
    assert_eq!(uri.bytes(), Bytes::from_static(b"http:"));
    assert!(uri.hier_part.path.origin.is_empty());
    assert!(uri.hier_part.authority.is_none());
}

#[test]
fn relatives_and_empty_parts() {
    let urn = Uri::parse_slice(b"urn:example:animal:ferret:nose").unwrap();
    assert_eq!(urn.scheme.unwrap().origin.as_ref(), b"urn");
    assert!(urn.hier_part.authority.is_none());
    assert_eq!(
        urn.hier_part.path.origin.as_ref(),
        b"example:animal:ferret:nose"
    );

    let host = Uri::parse_slice(b"//host").unwrap();
    assert!(host.scheme.is_none());
    assert_eq!(
        host.hier_part.authority.unwrap().host.bytes().as_ref(),
        b"host"
    );
    assert!(host.hier_part.path.origin.is_empty());

    let empty = Uri::parse_slice(b"").unwrap();
    assert!(empty.scheme.is_none());
    assert!(empty.hier_part.path.origin.is_empty());
    assert!(empty.query.is_none());
    assert!(empty.fragment.is_none());

    let frag = Uri::parse_slice(b"#frag").unwrap();
    assert_eq!(frag.fragment.unwrap().origin.as_ref(), b"frag");

    let query = Uri::parse_slice(b"?q").unwrap();
    assert_eq!(query.query.unwrap().origin.as_ref(), b"q");

    let file = Uri::parse_slice(b"file:///path").unwrap();
    let authority = file.hier_part.authority.as_ref().unwrap();
    assert_eq!(authority.host.bytes().as_ref(), b"");
    assert!(matches!(authority.host, Host::RegName(_)));
    assert_eq!(file.hier_part.path, Path::from_slice(b"/path").unwrap());
    let built = UriBuilder::new()
        .scheme(file.scheme.clone().unwrap())
        .hier_part(file.hier_part.clone())
        .build()
        .unwrap();
    assert_eq!(built.bytes().as_ref(), b"file:///path");

    let empty_port = Uri::parse_slice(b"http://host:/p").unwrap();
    assert_eq!(
        empty_port
            .hier_part
            .authority
            .as_ref()
            .unwrap()
            .port
            .as_ref()
            .unwrap()
            .origin
            .as_ref(),
        b""
    );
    assert_eq!(empty_port.hier_part.path.origin.as_ref(), b"/p");
}

#[test]
fn userinfo_ipv6_roundtrip() {
    let with_port = Uri::parse_slice(b"http://user:pass@[::1]:8080/p?q=1#f").unwrap();
    let authority = with_port.hier_part.authority.clone().unwrap();
    assert_eq!(
        authority.userinfo,
        Some(Userinfo::from_slice(b"user:pass").unwrap())
    );
    assert_eq!(authority.host.bytes().as_ref(), b"::1");
    assert_eq!(authority.port.unwrap().origin.as_ref(), b"8080");
    assert_eq!(with_port.hier_part.path.origin.as_ref(), b"/p");
    assert_eq!(with_port.query.unwrap().origin.as_ref(), b"q=1");
    assert_eq!(with_port.fragment.unwrap().origin.as_ref(), b"f");
    assert_eq!(
        UriBuilder::new()
            .scheme(Scheme::from_slice(b"http").unwrap())
            .hier_part(with_port.hier_part.clone())
            .query(Query::from_slice(b"q=1").unwrap())
            .fragment(Fragment::from_slice(b"f").unwrap())
            .build()
            .unwrap()
            .bytes()
            .as_ref(),
        b"http://user:pass@[::1]:8080/p?q=1#f"
    );
}

#[test]
fn pct_in_port_follows_find_at() {
    assert_eq!(
        Uri::parse_slice(b"http://host:%ZZ")
            .unwrap_err()
            .to_string(),
        "Invalid percent-encoding."
    );
    assert_eq!(
        Uri::parse_slice(b"http://host:%31")
            .unwrap_err()
            .to_string(),
        "Invalid port."
    );
    assert_eq!(
        Uri::parse_slice(b"http://user@host:%ZZ")
            .unwrap_err()
            .to_string(),
        "Invalid port."
    );
    assert_eq!(
        Uri::parse_slice(b"http://[%ZZ]/").unwrap_err().to_string(),
        "Invalid percent-encoding."
    );
    assert_eq!(
        Uri::parse_slice(b"http://user@[%ZZ]/")
            .unwrap_err()
            .to_string(),
        "Invalid host."
    );
    assert_eq!(
        Uri::parse_slice(b"http://[::1]@host")
            .unwrap_err()
            .to_string(),
        "Invalid userinfo."
    );
    assert_eq!(
        Uri::parse_slice(b"http://a b").unwrap_err().to_string(),
        "Invalid host."
    );
    assert_eq!(
        Uri::parse_slice(b"http://a b@h").unwrap_err().to_string(),
        "Invalid userinfo."
    );
}

#[test]
fn rejects_invalid_references() {
    assert!(Uri::parse_slice(b"http://example.com/%").is_err());
    assert!(Uri::parse_slice(b"http://example.com/%2").is_err());
    assert!(Uri::parse_slice(b"http://example.com/%ZZ").is_err());
    assert!(Uri::parse_slice(b"a@b:c").is_err());
    assert!(Uri::parse_slice(b"%66oo:bar").is_err());
    assert!(Scheme::from_slice(b"1http").is_err());
    assert!(Uri::parse_slice(b"http://[:::1]/").is_err());
    assert!(Uri::parse_slice(b"http://[1:2:3]/").is_err());
    assert!(Uri::parse_slice(b"http://[::1::2]/").is_err());
    assert!(Uri::parse_slice(b"http://[example.com]/").is_err());
    assert!(Uri::parse_slice(b"foo#bar#baz").is_err());
}

#[test]
fn relative_colon_in_later_segment() {
    let uri = Uri::parse_slice(b"foo/bar:baz").unwrap();
    assert!(uri.scheme.is_none());
    assert_eq!(uri.hier_part.path.origin.as_ref(), b"foo/bar:baz");
}

#[test]
fn builder_rejects_noscheme_colon() {
    let hier = HierPart::from_slice(b"foo:bar").unwrap();
    let built = UriBuilder::new().hier_part(hier).build();
    assert!(built.is_err());
}

#[test]
fn hier_part_builder_shares_path_bytes() {
    let hier = HierPartBuilder::new()
        .authority(Authority::from_slice(b"[::1]:80").unwrap())
        .path(Path::from_slice(b"/a").unwrap())
        .build()
        .unwrap();
    assert_eq!(hier.origin.as_ref(), b"//[::1]:80/a");
    assert_eq!(hier.authority.unwrap().host.bytes().as_ref(), b"::1");
    assert_eq!(hier.path.origin.as_ptr(), unsafe {
        hier.origin.as_ptr().add(b"//[::1]:80".len())
    });
}
