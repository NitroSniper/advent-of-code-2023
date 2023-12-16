use criterion::{black_box, criterion_group, criterion_main, Criterion};

use library::part2;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Part 2", |b| b.iter(|| part2::problem(black_box(include_str!("../src/data/input")))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
