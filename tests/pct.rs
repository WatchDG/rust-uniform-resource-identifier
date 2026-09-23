use bytes::{Bytes, BytesMut};
use uniform_resource_identifier::{
    decode_into, encode_into, encode_pairs, percent_decode, percent_encode, EncodeSet, Query, Uri,
};

#[test]
fn encode_unchanged_shares_buffer() {
    let raw = Bytes::from_static(b"abc-._~");
    let encoded = percent_encode(&raw, EncodeSet::QUERY_PAIR);
    assert_eq!(encoded.as_ptr(), raw.as_ptr());
}

#[test]
fn encode_space_and_delimiters() {
    let raw = Bytes::from_static(b"a b/&=");
    assert_eq!(
        percent_encode(&raw, EncodeSet::QUERY_PAIR).as_ref(),
        b"a%20b%2F%26%3D"
    );
    assert_eq!(percent_encode(&raw, EncodeSet::QUERY).as_ref(), b"a%20b/&=");
    assert_eq!(percent_encode(&raw, EncodeSet::PATH).as_ref(), b"a%20b/&=");
}

#[test]
fn encode_path_keeps_slash() {
    let raw = Bytes::from_static(b"a b/c");
    assert_eq!(percent_encode(&raw, EncodeSet::PATH).as_ref(), b"a%20b/c");
    assert_eq!(
        percent_encode(&raw, EncodeSet::SEGMENT).as_ref(),
        b"a%20b%2Fc"
    );
}

#[test]
fn encode_into_appends_exact_bytes() {
    let mut out = BytesMut::new();
    encode_into(b"a b/c", EncodeSet::SEGMENT, &mut out);
    assert_eq!(out.as_ref(), b"a%20b%2Fc");
}

#[test]
fn decode_hex_and_share() {
    let raw = Bytes::from_static(b"abc");
    let decoded = percent_decode(&raw).unwrap();
    assert_eq!(decoded.as_ptr(), raw.as_ptr());

    let encoded = Bytes::from_static(b"a%20b%2f");
    assert_eq!(percent_decode(&encoded).unwrap().as_ref(), b"a b/");
}

#[test]
fn decode_into_appends_decoded_bytes() {
    let mut out = BytesMut::new();
    decode_into(b"a%20b%2f", &mut out).unwrap();
    assert_eq!(out.as_ref(), b"a b/");
}

#[test]
fn decode_rejects_bad_pct() {
    assert!(percent_decode(&Bytes::from_static(b"%")).is_err());
    assert!(percent_decode(&Bytes::from_static(b"%2")).is_err());
    assert!(percent_decode(&Bytes::from_static(b"%ZZ")).is_err());
    assert!(percent_decode(&Bytes::from_static(b"%2G")).is_err());
    assert!(decode_into(b"%ZZ", &mut BytesMut::new()).is_err());
}

#[test]
fn pairs_split_without_copy() {
    let query = Query::from_slice(b"a=1&b&c=&=").unwrap();
    let base = query.origin.as_ptr();
    let pairs: Vec<_> = query.pairs().collect();
    assert_eq!(pairs.len(), 4);
    assert_eq!(pairs[0].0.as_ref(), b"a");
    assert_eq!(pairs[0].1.as_ref().unwrap().as_ref(), b"1");
    assert_eq!(pairs[0].0.as_ptr(), base);
    assert_eq!(pairs[1].0.as_ref(), b"b");
    assert!(pairs[1].1.is_none());
    assert_eq!(pairs[2].0.as_ref(), b"c");
    assert_eq!(pairs[2].1.as_ref().unwrap().as_ref(), b"");
    assert_eq!(pairs[3].0.as_ref(), b"");
    assert_eq!(pairs[3].1.as_ref().unwrap().as_ref(), b"");
}

#[test]
fn empty_query_has_no_pairs() {
    let query = Query::from_slice(b"").unwrap();
    assert!(query.pairs().next().is_none());
}

#[test]
fn encode_pairs_escapes_delimiters() {
    let encoded = encode_pairs([("a b", Some("c&d")), ("e", None), ("f", Some(""))]);
    assert_eq!(encoded.as_ref(), b"a%20b=c%26d&e&f=");
}

#[test]
fn query_pairs_roundtrip() {
    let encoded = encode_pairs([("a b", Some("c&d")), ("e", None)]);
    let mut raw = Vec::from(&b"http://h/?"[..]);
    raw.extend_from_slice(&encoded);
    let uri = Uri::parse(Bytes::from(raw)).unwrap();
    let query = uri.query.unwrap();
    let pairs: Vec<_> = query.pairs().collect();
    assert_eq!(pairs[0].0.as_ref(), b"a%20b");
    assert_eq!(
        percent_decode(pairs[0].1.as_ref().unwrap())
            .unwrap()
            .as_ref(),
        b"c&d"
    );
    assert_eq!(pairs[1].0.as_ref(), b"e");
    assert!(pairs[1].1.is_none());
}
