use criterion::{black_box, criterion_group, criterion_main, Criterion};

use library::part1;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Part 1", |b| b.iter(|| part1::problem(black_box(include_str!("../src/data/input_1")))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
