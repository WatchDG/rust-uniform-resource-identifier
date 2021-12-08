extern crate uniform_resource_identifier;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use uniform_resource_identifier::uri::scheme::parse_scheme;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_scheme");

    group
        .warm_up_time(Duration::from_secs(5))
        .measurement_time(Duration::from_secs(10))
        .sample_size(1000);

    group.bench_function("parse_scheme", |b| {
        b.iter(|| parse_scheme(black_box(b"https:"), black_box(&mut 0), black_box(&6)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
