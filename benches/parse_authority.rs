extern crate uniform_resource_identifier;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use uniform_resource_identifier::uri::authority::parse_authority;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_authority");

    let string = b"example.com";
    let end = string.len() - 1;

    group
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(10))
        .sample_size(1000);

    group.bench_function("parse_authority", |b| {
        b.iter(|| parse_authority(black_box(string), black_box(&mut 0), black_box(&end)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
