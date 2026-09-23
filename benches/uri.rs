use bytes::{Bytes, BytesMut};
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use uniform_resource_identifier::{
    decode_into, encode_into, encode_pairs, percent_decode, percent_encode, EncodeSet, Fragment,
    HierPart, Query, Scheme, Uri, UriBuilder,
};

const EXAMPLE: &[u8] = b"foo://example.com:8042/over/there?name=ferret#nose";
const USERINFO_PCT: &[u8] = b"http://user:pass%20x@example.com:8080/a%20b?q=1#f";
const IPV6: &[u8] = b"http://[2001:db8::1]:8080/path?q=1#f";
const UNRESERVED: &[u8] = b"abc-._~";
const PATH_RAW: &[u8] = b"a b/c";
const PCT: &[u8] = b"a%20b%2f";
const PAIRS: &[u8] = b"name=ferret&x=1";

fn long_uri() -> Vec<u8> {
    let mut uri = Vec::with_capacity(2200);
    uri.extend_from_slice(b"http://example.com/");
    while uri.len() < 2048 {
        uri.extend_from_slice(b"segment/");
    }
    uri.extend_from_slice(b"?name=ferret#nose");
    uri
}

fn bench_parse(c: &mut Criterion) {
    let input = Bytes::from_static(EXAMPLE);
    let long = Bytes::from(long_uri());
    let userinfo = Bytes::from_static(USERINFO_PCT);
    let ipv6 = Bytes::from_static(IPV6);
    let mut group = c.benchmark_group("parse");
    group.throughput(Throughput::Bytes(input.len() as u64));
    group.bench_function("parse", |b| {
        b.iter(|| black_box(Uri::parse(black_box(input.clone())).unwrap()))
    });
    group.bench_function("parse_slice", |b| {
        b.iter(|| black_box(Uri::parse_slice(black_box(EXAMPLE)).unwrap()))
    });
    group.throughput(Throughput::Bytes(long.len() as u64));
    group.bench_function("parse_long", |b| {
        b.iter(|| black_box(Uri::parse(black_box(long.clone())).unwrap()))
    });
    group.throughput(Throughput::Bytes(userinfo.len() as u64));
    group.bench_function("parse_userinfo", |b| {
        b.iter(|| black_box(Uri::parse(black_box(userinfo.clone())).unwrap()))
    });
    group.throughput(Throughput::Bytes(ipv6.len() as u64));
    group.bench_function("parse_ipv6", |b| {
        b.iter(|| black_box(Uri::parse(black_box(ipv6.clone())).unwrap()))
    });
    group.finish();
}

fn bench_percent(c: &mut Criterion) {
    let plain = Bytes::from_static(UNRESERVED);
    let raw = Bytes::from_static(PATH_RAW);
    let encoded = Bytes::from_static(PCT);

    let mut group = c.benchmark_group("percent_encode");
    group.throughput(Throughput::Bytes(plain.len() as u64));
    group.bench_function("unchanged", |b| {
        b.iter(|| black_box(percent_encode(black_box(&plain), EncodeSet::QUERY_PAIR)))
    });
    group.throughput(Throughput::Bytes(raw.len() as u64));
    group.bench_function("path", |b| {
        b.iter(|| black_box(percent_encode(black_box(&raw), EncodeSet::PATH)))
    });
    group.bench_function("encode_into", |b| {
        let mut out = BytesMut::new();
        b.iter(|| {
            out.clear();
            encode_into(black_box(PATH_RAW), EncodeSet::PATH, &mut out);
            black_box(out.len())
        })
    });
    group.finish();

    let mut group = c.benchmark_group("percent_decode");
    group.throughput(Throughput::Bytes(plain.len() as u64));
    group.bench_function("unchanged", |b| {
        b.iter(|| black_box(percent_decode(black_box(&plain)).unwrap()))
    });
    group.throughput(Throughput::Bytes(encoded.len() as u64));
    group.bench_function("pct", |b| {
        b.iter(|| black_box(percent_decode(black_box(&encoded)).unwrap()))
    });
    group.bench_function("decode_into", |b| {
        let mut out = BytesMut::new();
        b.iter(|| {
            out.clear();
            decode_into(black_box(PCT), &mut out).unwrap();
            black_box(out.len())
        })
    });
    group.finish();
}

fn bench_build(c: &mut Criterion) {
    let mut builder = UriBuilder::new();
    builder
        .scheme(Scheme::from_slice(b"foo").unwrap())
        .hier_part(HierPart::from_slice(b"//example.com:8042/over/there").unwrap())
        .query(Query::from_slice(b"name=ferret").unwrap())
        .fragment(Fragment::from_slice(b"nose").unwrap());
    let mut group = c.benchmark_group("build");
    group.throughput(Throughput::Bytes(EXAMPLE.len() as u64));
    group.bench_function("build", |b| b.iter(|| black_box(builder.build().unwrap())));
    group.finish();
}

fn bench_query(c: &mut Criterion) {
    let query = Query::from_slice(PAIRS).unwrap();
    let mut group = c.benchmark_group("query");
    group.throughput(Throughput::Bytes(PAIRS.len() as u64));
    group.bench_function("pairs", |b| {
        b.iter(|| {
            let mut size = 0usize;
            for (key, value) in query.pairs() {
                size += key.len();
                if let Some(value) = value {
                    size += value.len();
                }
            }
            black_box(size)
        })
    });
    group.throughput(Throughput::Bytes(
        (b"a b".len() + b"c&d".len() + b"e".len()) as u64,
    ));
    group.bench_function("encode_pairs", |b| {
        b.iter(|| black_box(encode_pairs([("a b", Some("c&d")), ("e", None)])))
    });
    group.finish();
}

criterion_group!(uri, bench_parse, bench_percent, bench_build, bench_query);
criterion_main!(uri);
