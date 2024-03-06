use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gnuplot::{Caption, Color, Figure};

fn criterion_benchmark(c: &mut Criterion) {
    fibonacci(20);
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("fib 20", |b| b.iter(|| fibonacciImproved(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
