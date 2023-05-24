use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

pub fn from_nanos(c: &mut Criterion) {
    static MULTIPLIER: u64 = u32::MAX as u64;
    for dur in [0, MULTIPLIER, 6000 * MULTIPLIER] {
        c.bench_with_input(BenchmarkId::new("Duration", dur), &dur, |b, &ns| {
            b.iter(|| black_box(Duration::from_nanos(ns)))
        });
    }
}

pub fn roundtrip(c: &mut Criterion) {
    static MULTIPLIER: u64 = u32::MAX as u64;
    for dur in [0, MULTIPLIER, 6000 * MULTIPLIER] {
        c.bench_with_input(BenchmarkId::new("Duration to ns", dur), &dur, |b, &ns| {
            b.iter(|| black_box(black_box(Duration::from_nanos(ns)).as_nanos()))
        });
    }
}

criterion_group!(benches, from_nanos, roundtrip);
criterion_main!(benches);
