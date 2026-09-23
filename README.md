# rust-uniform-resource-identifier

[RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986) URI references for Rust. Parsing slices one `bytes::Bytes` buffer into components. Building writes into one buffer.

```toml
[dependencies]
uniform-resource-identifier = "0.0.13"
bytes = "1"
```

## Example

```rust
use bytes::Bytes;
use uniform_resource_identifier::{
    encode_pairs, percent_decode, percent_encode, EncodeSet, Fragment, HierPart, Host, Query,
    Scheme, Uri, UriBuilder,
};

fn main() -> Result<(), uniform_resource_identifier::UriError> {
    let uri = Uri::parse(Bytes::from_static(
        b"foo://example.com:8042/over/there?name=ferret#nose",
    ))?;

    assert_eq!(uri.scheme.as_ref().unwrap().origin.as_ref(), b"foo");
    let authority = uri.hier_part.authority.as_ref().unwrap();
    assert!(matches!(authority.host, Host::RegName(_)));
    assert_eq!(authority.host.bytes().as_ref(), b"example.com");
    assert_eq!(authority.port.as_ref().unwrap().origin.as_ref(), b"8042");
    assert_eq!(uri.hier_part.path.origin.as_ref(), b"/over/there");
    assert_eq!(uri.query.as_ref().unwrap().origin.as_ref(), b"name=ferret");
    assert_eq!(uri.fragment.as_ref().unwrap().origin.as_ref(), b"nose");

    let query = encode_pairs([("name", Some("a b")), ("q", Some("c&d"))]);
    assert_eq!(query.as_ref(), b"name=a%20b&q=c%26d");

    let built = UriBuilder::new()
        .scheme(Scheme::from_slice(b"foo")?)
        .hier_part(HierPart::from_slice(b"//example.com:8042/over/there")?)
        .query(Query::from_bytes(query)?)
        .fragment(Fragment::from_slice(b"nose")?)
        .build()?;
    assert_eq!(
        built.bytes().as_ref(),
        b"foo://example.com:8042/over/there?name=a%20b&q=c%26d#nose"
    );

    let raw = Bytes::from_static(b"a b");
    let encoded = percent_encode(&raw, EncodeSet::PATH);
    assert_eq!(encoded.as_ref(), b"a%20b");
    assert_eq!(percent_decode(&encoded)?.as_ref(), b"a b");
    Ok(())
}
```
