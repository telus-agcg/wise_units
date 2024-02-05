#[macro_use]
extern crate criterion;

use criterion::{black_box, Criterion};
use wise_units::parser::parse;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("og-parse: m", |b| b.iter(|| parse(black_box("m"))));
    c.bench_function("og-parse: magain", |b| b.iter(|| parse("m")));

    c.bench_function("og-parse: m2", |b| b.iter(|| parse(black_box("m2"))));
    c.bench_function("og-parse: 2m", |b| b.iter(|| parse(black_box("2m"))));
    c.bench_function("og-parse: 2m2", |b| b.iter(|| parse(black_box("2m2"))));
    c.bench_function("og-parse: 2m2{things}", |b| {
        b.iter(|| parse(black_box("2m2{things}")))
    });

    c.bench_function("og-parse: km", |b| b.iter(|| parse(black_box("km"))));
    c.bench_function("og-parse: km2", |b| b.iter(|| parse(black_box("km2"))));
    c.bench_function("og-parse: 2km", |b| b.iter(|| parse(black_box("2km"))));
    c.bench_function("og-parse: 2km2", |b| b.iter(|| parse(black_box("2km2"))));
    c.bench_function("og-parse: 2km2{things}", |b| {
        b.iter(|| parse(black_box("2km2{things}")))
    });

    c.bench_function("og-parse: g.m", |b| b.iter(|| parse(black_box("g.m"))));
    c.bench_function("og-parse: g2.m", |b| b.iter(|| parse(black_box("g2.m"))));
    c.bench_function("og-parse: 2g.m", |b| b.iter(|| parse(black_box("2g.m"))));
    c.bench_function("og-parse: 2g2.m", |b| b.iter(|| parse(black_box("2g2.m"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
