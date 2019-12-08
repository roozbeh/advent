#![feature(test)]
include!("../src/solution.rs");

extern crate test;

const INPUT: &'static str = include_str!("../image");

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("p1_me", |b| b.iter(|| part1(INPUT, 25, 6)));
    c.bench_function("p2_me", |b| b.iter(|| part2(INPUT, 25, 6)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
);
criterion_main!(benches);
